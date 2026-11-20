#!/bin/sh
set -eu

cd /app

# Render the runtime config at container start. The API origin is read from
# process.env (never interpolated into shell code), so it cannot inject shell
# commands and is never written to an image layer or echoed to the logs.
# Everything runs in one process: the adapter's asset table is patched in
# memory and build/index.js is then imported, which builds its static file map
# from that same table. A second process would load the unpatched table.
node --input-type=module -e '
import fs from "node:fs";
import crypto from "node:crypto";
import { execFileSync } from "node:child_process";
import { assets } from "./build/adapter-node.js";

const config = {
  apiBaseUrl: process.env.FLORA_API_BASE_URL || "http://localhost:3000",
  wsBaseUrl: null
};
const source = "globalThis.__FLORA_CONFIG__ = " + JSON.stringify(config) + ";\n";
fs.writeFileSync("build/config.js", source);
// SvelteKit adapter-node serves /config.js from build/client, so the browser
// must receive the same generated file there rather than the committed default.
fs.mkdirSync("build/client", { recursive: true });
fs.writeFileSync("build/client/config.js", source);

// The committed build ships pre-compressed variants of config.js. They are
// stale after regeneration above, so drop them: a stale variant would carry
// the old compressed size and break clients on a mismatched Content-Length.
execFileSync("rm", ["-f", "build/config.js.br", "build/config.js.gz", "build/client/config.js.br", "build/client/config.js.gz"]);

// adapter-node builds its static file map from this table when build/index.js
// is imported below, so correct the /config.js metadata in place first.
const entry = assets.entries.find(([key]) => key === "/config.js")[1];
entry.size = Buffer.byteLength(source);
entry.etag = crypto.createHash("sha256").update(source).digest("base64url");
delete entry.gz;
delete entry.br;

await import("./build/index.js");
'

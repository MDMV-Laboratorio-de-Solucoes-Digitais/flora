# 034 — WebSocket `Origin` validation: security-grounded, multilingual primary-source research

**Date:** 2026-11-16. **Repo:** `flora` (archived checkout, canonical `github/dev@1be723cc`).
**Type:** research/design only — no source, config, env, workflow, or test file changed. No live probe was run in this audit.
**Builds on / refreshes:** `docs/research/2026-11-16-034-origin-boundary-exact-plan.md` (§0–§11), `…-033-origin-boundary-plan.md`, `…-032-cors-final-plan.md`, `…-030-cors-allowlist-plan.md`.
**Scope:** security policy decision for the WS `Origin` guard proposed in `websocket.rs`, reusing `app.cors_origins`. Seven languages: EN, pt-BR, es, de, fr, zh-CN, ja.

---

## 0. Verdict (one paragraph)

**GO** for a hybrid policy: **fail-closed for any present `Origin` that is not an exact allowlist entry (includes `Origin: null`), fail-open for an absent `Origin`**, using the same `app.cors_origins` list as `cors_layer`, compared as an exact byte string, checked before auth and returning `403`. This is standards-backed (RFC 6455 §10.2; MDN; OWASP) and closes the *browser-based* CSWSH / cross-origin-use case. It is **not** an authentication or tenancy control (OWASP ASVS V14.5.2), **not** a fix for `?token=` leakage, and **not** a browser-enforcement proof. With Bearer-token auth the classic cookie-driven CSWSH precondition is already absent, so the guard is defense-in-depth plus an explicit cross-origin boundary — record that honestly in the ADR.

---

## 1. Is checking `Origin` sufficient to stop CSWSH?

**Short answer: no, not by itself — and for Flora's Bearer-token auth it is not the primary control.**

Evidence chain:

- **RFC 6455 §1.3 / §4.1(8)** [E1]: the `Origin` header "is used to protect against unauthorized cross-origin use of a WebSocket server by scripts using the WebSocket API in a web browser"; browser clients **MUST** send it, non-browser clients **MAY**. It is the ASCII serialization of the origin of the code establishing the connection.
- **RFC 6455 §10.1 (Non-Browser Clients)** [E1]: the protocol "protects against malicious JavaScript running inside a trusted application such as a web browser … by checking of the `Origin` header". Dedicated clients "can … send fake `Origin` header fields, misleading the server"; "the intent is not to prevent non-browsers from establishing connections".
- **RFC 6455 §10.2 (Origin Considerations)** [E1]: servers intended for certain sites only **SHOULD** verify `Origin` is expected and, if unacceptable, **SHOULD** reply with **HTTP 403 Forbidden**. The threat described is the untrusted *author of JavaScript executing in a trusted client* — not a token thief.
- **RFC 6455 §10.5** [E1]: WS prescribes no auth mechanism; cookies, HTTP auth, TLS auth are examples. **Classic CSWSH needs an ambient credential the browser attaches automatically** (OWASP's 4-step cookie scenario [E5]). A Bearer token in an `Authorization` header is *not* ambient: a malicious cross-site script cannot read the victim's token (absent XSS), and cannot set that header cross-origin (Fetch forbids header manipulation; `Authorization` is a "CORS non-wildcard request-header" [E9]).
- **MDN Writing WebSocket servers** [E3]: "All **browsers** send an `Origin` header. You can use this header for security … and send a **403 Forbidden** if you don't like what you see. This is effective against Cross Site WebSocket Hijacking (CSWH). However, be warned that **non-browser agents can send a faked `Origin`**. **Most applications reject requests without this header.**"
- **OWASP WebSocket Security Cheat Sheet** [E5]: "Validate the `Origin` header on every handshake. Always use an explicit allowlist of trusted origins. Browsers include this header and malicious JavaScript cannot override it"; "**Use an allowlist, not a denylist. Avoid wildcards or substring matching which are error-prone**"; real-world Gitpod CSWSH (2023) was "insufficient origin validation". CSRF tokens and `SameSite` cookies are listed as **additional** protections.
- **OWASP WSTG 10-Testing WebSockets** [E6]: "It is the server's responsibility to verify the `Origin` header in the initial HTTP WebSocket handshake … the WebSocket server may accept connections from any origin", enabling "CSRF-like issues … Cross-Site Websocket Hijacking (CSWH or CSWSH)".
- **Counterweight (must be recorded):** OWASP ASVS Cornucopia V14.5.2 [E8]: "Verify that the supplied Origin header is **not used for authentication or access control decisions**, as the Origin header can easily be changed by an attacker." OWASP HTML5 Cheat Sheet [E7]: "**Don't rely only on the `Origin` header for Access Control checks.** Browser always sends this header in CORS requests, but may be spoofed outside the browser. Application-level protocols should be used to protect sensitive data."

**Conclusion for Flora:** the real access-control boundary is `verify_bearer_token` + org ownership (`websocket.rs:170-178, 200-211`). The `Origin` guard:
1. satisfies RFC 6455 §10.2 / MDN / OWASP for browser-originated cross-site use;
2. is meaningful defense-in-depth even with header-Bearer auth (blocks a same-origin XSS-adjacent script from another allowed browser tab from silently opening a WS with a stolen-but-in-memory token, and blocks accidental cross-origin embedding);
3. **cannot** stop an attacker who already holds a valid token (header **or** query) — they simply omit or forge `Origin`, which RFC 6455 §10.1 explicitly permits for non-browsers.

**Multilingual cross-check (MDN Writing WebSocket servers, all 7 locales fetched 2026-11-16):**

| Locale | CSWH / 403 / "non-browser can fake" / "most apps reject without" note | Note |
|---|---|---|
| EN [E3] | present, verbatim | baseline |
| pt-BR [E13a] | present ("…envie uma 403 Forbidden… agentes non-browser podem apenas enviar uma falsa Origin. Muitas aplicações vão rejeitar requisições sem cabeçalho.") | community translation |
| es [E13b] | present ("…enviar un 403 Forbidden… agentes no navegadores pueden enviar un falso Origin. La mayoría de las aplicaciones rechazaran las solicitudes sin esta cabecera.") | community translation |
| de [E13c] | present ("…403 Forbidden … Cross Site WebSocket Hijacking (CSWH) … Nicht-Browser-Agenten einen gefälschten Origin … Die meisten Anwendungen lehnen Anfragen ohne diesen Header ab.") | marked **machine-translated** by MDN |
| ja [E13d] | present **and adds a CWE-1385 link** ("クロスサイト WebSocket ハイジャッキング (CSWH)") | community translation; extra reference |
| zh-CN [E13e] | present ("…发送一个403 Forbidden… 非浏览器只能发送一个模拟的 Origin。大多数应用会拒绝不含这个请求头的请求") | community translation |
| **fr [E13f]** | **GAP — the Origin/403/CSWH note is absent** in the fetched FR page; the "Requête de poignée de mains côté client" section jumps from the handshake example to a duplicated `Sec-WebSocket-Accept` note. FR also renders `example.com8000` / `example.comchat` (missing `:`). | translation drift; **do not cite FR for this claim** |

**Gap statement:** the security claim is consistent in 6/7 locales; FR is incomplete. All authoritative force comes from EN + RFCs + OWASP, not from translations. No non-EN source contradicts the EN guidance.

---

## 2. Policy per origin case — and what the ADR must record

| Case | Recommended decision | Basis / trade-off |
|---|---|---|
| **Absent `Origin`** | **ALLOW** (fail-open for this case only) | RFC 6455 §4.1(8): browser MUST send, non-browser MAY; §10.2: "intent is not to prevent non-browsers from establishing connections" [E1]. Needed for native/Tauri clients and DB-free curl/Rust oneshot tests. **Cost:** the guard gives zero protection to any client that simply omits the header — but that attacker still needs a valid token. MDN/OWASP note most apps reject absent `Origin` [E3][E5]; fail-closed is a valid alternative but would force every native client and proxy path to set it, and a proxy that strips `Origin` would break all connections. |
| **`Origin: null`** | **DENY (403)** unless ops explicitly allowlists the literal `"null"` | Opaque origin serialized as `null` by RFC 6454 §6.1/§6.2 [E2]; MDN lists sandbox, `data:`/`file:`, cross-origin redirects, network errors [E4]. Deny-by-default is safe; if a sandboxed iframe or a particular webview emits `null`, the webview test must prove it before ops adds the literal. `AppConfig::validate()` does **not** currently forbid `"null"` [L3], so allowlisting is possible but must be an explicit, recorded decision. |
| **Present but unlisted** | **DENY (403)** | Explicit allowlist, not denylist — OWASP [E5]; RFC 6455 §10.2 "unacceptable → 403" [E1]; MDN 403 guidance [E3]. |
| **Subdomains** | **No implicit subdomain matching** — each origin configured literally | RFC 6454 §3.2 explicitly rejects "registry-controlled domain" grouping; OWASP example: `indexOf(".owasp.org")` lets `owasp.org.attacker.com` pass [E7]. `app.example.com` does **not** match an allowlisted `example.com`. Trade-off = ops burden vs. a real bypass class. |
| **Multiple configured origins** | `allowed.iter().any(|e| e == value)` — one entry may match; one list serves CORS and WS | Keeps `cors_layer` and the WS guard from disagreeing [L2][L4]. RFC 6454 §7.1 allows a space-separated `origin-list`, but browsers send one serialized origin on a WS handshake; exact match will reject a multi-origin value even if one entry is listed — record as an accepted limitation (fails closed). |

**Fail-open vs fail-closed summary:** the proposed design is *hybrid* — fail-closed on values, fail-open on absence. Pure fail-closed (deny absent) is stronger against a token-less attacker only in that it reduces signal surface; it breaks native interop and makes proxy stripping an outage instead of a silent bypass. Pure fail-open (allow any present origin matching a suffix) is unacceptable per OWASP.

**The ADR must record, explicitly (NV-2/NV-3/NV-4/NV-6 from [L4] plus this research):**
1. The exact policy matrix above, including `null` and absent.
2. The threat model: **browser-based cross-origin use / CSWSH only**; not non-browser token theft, not tenancy, not authorization.
3. That `verify_bearer_token` + org ownership remain the real boundary (OWASP ASVS V14.5.2 [E8]).
4. The one-list invariant (`app.cors_origins` feeds both CORS and WS; no second list).
5. The `?token=` residual risk and the proxy-forwarding assumption as named NVs, not as solved problems.

---

## 3. Exact-match comparison pitfalls

Serialization is defined by **RFC 6454 §4, §5, §6.2, §7.1–7.3** [E2] and MDN Origin syntax [E4]:

- **Scheme and host are lowercased**; default ports are **omitted**. A browser sends `http://example.com`, **never** `http://example.com:80` or `https://example.com:443`. Configuring the `:80`/`:443` form silently never matches. Same for a trailing `/`, a path, or a query — origin serialization has none.
- **Scheme is part of origin** (RFC 6454 §3.2.1): `http://host` ≠ `https://host`, and `ws(s)://` is not the browser's Origin scheme for an `http(s)` page.
- **IPv6** keeps the RFC 3986 bracketed host: `http://[::1]:5173`. Brackets are mandatory in the config literal. `localhost`, `127.0.0.1`, and `[::1]` are three distinct origins.
- **Case sensitivity:** `HeaderValue` equality compares the underlying `Bytes` byte-for-byte — `impl PartialEq` delegates to `self.inner == other.inner`; `PartialEq<str>`/`PartialEq<String>` also compare raw bytes; there is no case folding for values, and sensitivity flags are explicitly excluded from equality [E11]. `AppConfig::validate()` does not normalize entries [L3], so `HTTP://LOCALHOST:5173` in config will never match.
- **HeaderValue vs String:** the planned guard uses `entry.as_str() == value` (`String`/`&str`, byte-exact) [L4]; tower-http's `AllowOrigin::list` uses `origin.filter(|o| l.contains(o))` on `Vec<HeaderValue>` [E10], which is the same byte-exact comparison [E11]. Keeping both exact is deliberate — CORS and WS must agree on the same literal. Non-UTF-8 header values return `Err` from `to_str()` and must deny (fail closed).
- **`*` literal:** `AllowOrigin::list` **panics** on `*` [E10]; `AppConfig::validate()` rejects any `*` substring first [L3]; the WS guard would treat `*` as a literal and simply never match. No new panic path.
- **Whitespace / duplicates:** config validation rejects empty entries but not trailing spaces, trailing slashes, or duplicate entries [L3]. A typo becomes a silent 403 for that client. Ops must configure exact serialized origins; a startup smoke with the real webview is the only reliable check.
- **Tauri custom schemes** (`tauri://localhost`, `http://tauri.localhost`, `https://tauri.localhost`) are not ordinary HTTP serialized origins in every webview; they must be entered literally and verified per OS. Defaults do **not** include them [L3].

**Top 3 pitfalls:** (1) default-port/trailing-slash/case literals that never match; (2) assuming subdomains or `null` are covered by an exact list; (3) a proxy stripping `Origin`, which with fail-open-on-absent makes the guard silently inert.

---

## 4. Proving behavior from a non-browser client — and its limits

**What to run** (see [L4] §6–§7 for the exact commands; all are read-only/no-DB for the guard itself):

- Pure Rust units on `origin_allowed(&HeaderMap, &[String])`: allowlisted → true; `https://evil.example` and `null` → false; empty `HeaderMap` → true [L4].
- DB-free Axum `oneshot` with a handcrafted upgrade request (`Connection: Upgrade`, `Upgrade: websocket`, `Sec-WebSocket-Version: 13`, `Sec-WebSocket-Key`, optional `Origin`) over `connect_lazy` pool: bad origin → **403** before auth; allowed origin + no token → **401** after the guard [L4].
- `curl -si` with the same upgrade headers and `-H "Origin: …"` shows the status line without needing a WS data frame; `websocat`/`wscat --header "Origin: …"` can complete a real handshake.
- CORS side: `OPTIONS` preflight shows ACAO echoed for allowed, absent ACAO for disallowed (tower-http still answers `200` to OPTIONS [L4]).

**What this proves:** the server-side predicate is wired into the handler before auth/DB; exact-string semantics for the tested literals; `403` vs `401` distinction; CORS/WS consistency over the configured list; no dependency on Postgres for the guard.

**What this does NOT prove:**
- that any browser/webview actually sends the literal you configured (only a real browser/webview test does);
- that a browser would *block* anything — **curl/websocat do not enforce CORS** [L4];
- that a reverse proxy forwards `Origin` on the real VPS path (capture at the app is required);
- that CSWSH is impossible, or that a stolen `?token=` cannot be replayed from a non-browser client (RFC 6455 §10.1 [E1] says it can);
- that Tauri uses the assumed scheme on every OS.

Record this boundary verbatim in the evidence: "non-browser proof = server logic; T7 / real-webview gate = browser behavior."

---

## 5. Interaction with `?token=` query auth

`resolve_ws_token` prefers `Authorization: Bearer …` and falls back to `params.token` [L1]. The `Origin` guard **leaves the query-token leak risk unchanged**:

- The URL token still reaches proxy/access logs, server logs, browser history, and possibly `Referer`; a bearer in a header does not. OWASP's cookie-CSWSH scenario [E5] does not apply here, but query-token exposure is a distinct issue already flagged as 030 NV-6 / 032 §9 / 034 NV-5 [L4].
- The guard does not make query-token replay harder: a holder can send the token with **no** `Origin` (allowed by our policy) or a forged one [E1].
- **Hardening worth recording as NV (no scope creep in cycle 034):**
  - NV: prefer header token for every client that can set it (native/Tauri, non-browser tools); keep the query fallback only because the browser `WebSocket` API cannot set arbitrary headers.
  - NV: record token TTL (session `3600`s), rotation on `select-organization`, and that access logs/proxy logs must **redact** the `token` query parameter; verify the effective proxy log format on the VPS.
  - NV: do not log the `Authorization` header or query token in tracing (current code logs only user/org IDs [L1]).
  - Do **not** change the auth scheme, token storage, or `WsTokenQuery` in cycle 034 — file the above as a separate assessment.

---

## 6. Reverse-proxy requirements and cheap-VPS implications

- RFC 6455 §4.2 [E1]: a proxy may be part of the "server"; all infrastructure from TCP termination to the app counts. §1.7/§10.6 [E1]: WS is independent TCP over HTTP Upgrade; TLS (`wss`) is the recommended confidentiality control.
- OWASP WS Cheat Sheet [E5]: proxies/load balancers "must be configured to handle WebSocket upgrades": HTTP/1.1 upgrade, pass `Upgrade` and `Connection: upgrade`, proper read timeouts; if WAF does not inspect WS, rely on server-side validation and app logging.
- **`Origin` is an ordinary request header.** Most proxies forward it, but a config that rewrites/drops headers (`proxy_set_header` overriding `Origin`, or a sanitizing WAF) can strip it. With our fail-open-on-absent policy, stripping does **not** break connections — it makes the guard **silently inert for browser traffic while looking healthy**. That is the single most important operational NV.
- **Required on the VPS:** (1) confirm `Upgrade`/`Connection` reach the app; (2) confirm the app sees the browser's `Origin` (log/capture during T7); (3) do not confuse `Host`, `X-Forwarded-*`, or the proxy's own scheme with `Origin` — `Origin` reflects the *page's* scheme/host/port, not the upstream. If the frontend is on a non-default port (`http://example.com:4173`), that **port is part of the origin** and must be in the allowlist; if TLS terminates at 443 with path routing, the origin is `https://example.com` with no port.
- Config is startup-only: allowlist changes require an API restart [L3][L4]. A committed dev-default `static/config.js` silently used if deploy forgets to render it is a separate ops check [L4].

---

## 7. Evidence map

### Primary external (fetched 2026-11-16)

| ID | Source | Lang | URL | Claim used |
|---|---|---|---|---|
| E1 | RFC 6455 (IETF, Fette & Melnikov, Dec 2011) | EN | https://www.rfc-editor.org/rfc/rfc6455.html | §1.3/§4.1(8) Origin MUST from browsers, MAY from non-browsers, anti-cross-origin-use purpose; §1.6 origin model useless for capable clients; §10.1 non-browsers can fake Origin; §10.2 verify expected origin, else **403**; §10.5 no prescribed auth (cookies/HTTP/TLS); §4.2 proxies are part of "server"; §10.6 TLS (`wss`). |
| E2 | RFC 6454 (IETF, Barth, Dec 2011) | EN | https://www.rfc-editor.org/rfc/rfc6454.html | §3.2 scheme/host/port tuple, rejects registry-domain grouping; §3.2.1 same-origin examples incl. `:80` equivalence; §4 host lowercased, default port substituted; §5 triple equality; §6.1/§6.2 serialization omits default port, returns `null` for opaque origins; §7.1 `origin-list` grammar; §7.3 at most one Origin header field. |
| E3 | MDN — Writing WebSocket servers | EN | https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers | All browsers send Origin; use for security, 403 for disliked origin; effective against CSWH; non-browser agents can fake it; most applications reject requests without it; reverse proxies commonly pre-process handshakes. |
| E4 | MDN — Origin header | EN | https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Origin | Syntax `<scheme>://<hostname>[:<port>]` or `null`; forbidden request header; default port implied; `null` cases (opaque origins, sandbox, redirects, `data:`/`file:`). |
| E5 | OWASP — WebSocket Security Cheat Sheet | EN | https://cheatsheetseries.owasp.org/cheatsheets/WebSocket_Security_Cheat_Sheet.html | Validate Origin on every handshake; explicit allowlist, not denylist; avoid wildcards/substring (Gitpod 2023 CSWSH); malicious JS cannot override browser-sent Origin; CSRF tokens/SameSite as additional protections; proxy must pass Upgrade/Connection, support HTTP/1.1, timeouts; WAF gap. |
| E6 | OWASP — WSTG 4-Web…/11-Client-side Testing/10-Testing WebSockets | EN | https://owasp.org/www-project-web-security-testing-guide/latest/4-Web_Application_Security_Testing/11-Client-side_Testing/10-Testing_WebSockets | Server responsibility to verify Origin; no validation → any origin → CSRF-like CSWH; black-box test = connect with a WS client from an unauthorized origin. |
| E7 | OWASP — HTML5 Security Cheat Sheet (CORS section) | EN | https://cheatsheetseries.owasp.org/cheatsheets/HTML5_Security_Cheat_Sheet.html | Allow only selected trusted ACAO domains; substring matching (`indexOf(".owasp.org")`) lets `owasp.org.attacker.com` pass; **do not rely only on Origin for access control** — spoofable outside the browser. |
| E8 | OWASP — Cornucopia, ASVS 4.0.3 V14.5.2 | EN | https://cornucopia.owasp.org/taxonomy/asvs-4.0.3/14-configuration/05-http-request-header-validation | "Origin header is not used for authentication or access control decisions, as the Origin header can easily be changed by an attacker" (CWE-346). |
| E9 | WHATWG — Fetch Standard (Living Standard, updated 2026-11-14) | EN | https://fetch.spec.whatwg.org/#origin-header | `Origin` is a forbidden request header (UA-controlled); `Authorization` is a CORS non-wildcard request-header; CORS protocol and credentials semantics; supersedes RFC 6454 Origin semantics. |
| E10 | tower-http 0.7.0 — `allow_origin.rs` source | EN | https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_origin.rs.html | `OriginInner::List(l) => origin.filter(|o| l.contains(o))` = exact `Vec<HeaderValue>` membership; `list()` panics on `*`; `mirror_request()`/`predicate(true)` = reflection (not used). |
| E11 | http 1.3.1 — `header/value.rs` source | EN | https://docs.rs/http/1.3.1/src/http/header/value.rs.html | `HeaderValue::eq` compares inner `Bytes` byte-for-byte; `PartialEq<str>`/`PartialEq<String>` compile to raw byte comparison; sensitivity not factored into equality; `to_str()` rejects non-visible ASCII. |
| E12 | axum 0.8.9 — `WebSocketUpgrade` docs | EN | https://docs.rs/axum/0.8.9/axum/extract/ws/struct.WebSocketUpgrade.html | Extractor requires GET (HTTP/1.1), exposes subprotocol/frame-size APIs, `on_upgrade()` finalizes 101; **no Origin validation performed by axum**. |
| E13a–f | MDN — Writing WebSocket servers, localized | pt-BR / es / de / ja / zh-CN / fr | https://developer.mozilla.org/pt-BR/docs/Web/API/WebSockets_API/Writing_WebSocket_servers ; /es/… ; /de/… ; /ja/… ; /zh-CN/… ; /fr/… | Cross-check of the CSWH/403/fake-Origin/reject-absent note. Present in pt-BR, es, de, ja, zh-CN (ja adds CWE-1385); **absent in fr** (see §1 gap table). |

### Local / code (read-only)

| ID | Location | Claim |
|---|---|---|
| L1 | `flora-api/src/routes/websocket.rs:105-179,200-211` | `WsTokenQuery.token`, header-first `resolve_ws_token`, `extract_org_context` → `verify_bearer_token`, nil-org rejection, org channel ownership; no `Origin` check today. |
| L2 | `flora-api/src/middleware/cors.rs:26-55` | `AllowOrigin::list`, explicit methods/headers, no credentials, 24h max-age, derives `Vary: Origin`. |
| L3 | `flora-core/src/config/app.rs:18-19,46-51,78-132` | `cors_origins` list, 4 loopback defaults (no Tauri), validation rejects empty/`*`/invalid HeaderValue; does not normalize case/slash or reject `null`. |
| L4 | `docs/research/2026-11-16-034-origin-boundary-exact-plan.md` | Exact E6 guard code, test plan, live-proof commands, NV-1…NV-13, stale-binary blocker. |
| L5 | `docs/research/2026-11-16-033-origin-boundary-plan.md` | Primary-source set S1–S15, code facts C1–C15, policy rationale. |
| L6 | `docs/research/2026-11-16-032-cors-final-plan.md`, `…-030-cors-allowlist-plan.md` | Prior CORS/WS decisions: CORS ≠ cross-origin boundary; WS needs explicit Origin check; `?token=` assessment owed. |

---

## 8. Named vulnerabilities / unknowns (NV labels)

| ID | Sev | NV |
|---|---|---|
| NV-1 | H | **Language gap:** FR MDN localization omits the Origin/403/CSWH note; do not use it as evidence. EN + RFCs + OWASP are authoritative (§1). |
| NV-2 | H | **Origin is not access control.** OWASP ASVS V14.5.2 [E8] and HTML5 CS [E7]: spoofable outside the browser. ADR must name `verify_bearer_token` + org ownership as the real boundary. |
| NV-3 | M | **Fail-open on absent Origin** is an explicit trade-off: interop/native client support vs. a guard that any curl client can bypass by omitting the header. Must be an ADR decision, not an accident. |
| NV-4 | M | **Proxy `Origin` stripping** makes the guard silently inert while connections keep working (fail-open). Verify at the app during T7; cheap-VPS nginx/Caddy config is in scope for ops verification. |
| NV-5 | M | **`?token=` leakage unchanged** by the Origin guard (§5). Header-first preference exists; query fallback remains because browser WS API cannot set headers. Separate assessment owed. |
| NV-6 | M | **Config literal drift** — default ports, trailing slash, uppercase, spaces, IPv6 brackets, Tauri scheme literals are not normalized or validated [L3][E2][E11]; wrong literal = silent 403 for that client. |
| NV-7 | M | **`null` and subdomains** must not be implicitly trusted; exact match means a space-separated `origin-list` value (RFC 6454 §7.1) is rejected even if one entry matches — accepted, fails closed. |
| NV-8 | M | **Tauri default gap** — Tauri origins absent from defaults [L3]; webview must prove the exact serialized literal per OS before ops adds it; CSP `connect-src` is a separate gate. |
| NV-9 | M | **Live binary is stale** (034 NV-1 [L4]): wildcard CORS on `localhost:3000`. No live proof is honest until a canonical rebuild/restart; this audit ran no live probe. |
| NV-10 | L | **Non-browser proof scope** — curl/websocat/oneshot prove server logic only; they do not enforce CORS or prove browser/proxy behavior (§4). |
| NV-11 | L | Config is startup-only; allowlist changes require restart; committed dev-default `config.js` needs an ops render check [L4]. |

---

## 9. What can honestly be claimed after implementation

**Can claim (with test/live evidence):**
- "The WS handshake applies an exact-match `Origin` allowlist from `app.cors_origins`; present-but-unlisted and `null` origins receive 403 before auth/DB; absent `Origin` is allowed."
- "CORS and WS enforce the same literal list; no wildcard, no credentials, no substring matching."
- "Deterministic Rust tests cover allowed/disallowed/`null`/absent cases without a database; a real browser/webview smoke confirms the configured webview origin passes."
- "Server-side behavior was verified from a non-browser client for the tested header values."

**Cannot claim (say so explicitly):**
- "CSWSH is impossible" — a holder of a valid token can connect from any origin/non-browser and forge or omit `Origin` [E1].
- "Browsers are being blocked by our guard" — that is the browser's CORS/SOP job; curl does not enforce it [L4].
- "The `?token=` leak is fixed" — it is not (NV-5).
- "All seven locales support this guidance" — FR translation is incomplete (NV-1).
- "The proxy forwards `Origin` on the VPS" — until captured at the app (NV-4).
- "ADR-0002 #2 is met" — only after green gates, canonical rebuild, T8 live proof, T7 browser regression, and the ADR note [L4].

---

## 10. Compact verdict

**Recommended policy:** hybrid — allow absent `Origin`; deny `null` and any present-but-unlisted origin with **403** before auth; exact byte match against the single `app.cors_origins` list shared with CORS; no subdomain/suffix matching. **GO**, with the ADR explicitly recording that this is a browser-origin boundary, not authentication or tenancy.
**Top 3 pitfalls:** (1) default-port / trailing-slash / case / IPv6-bracket / Tauri-scheme literal drift → silent 403; (2) proxy stripping `Origin` → guard silently inert under fail-open-on-absent; (3) treating the guard as a CSWSH-complete fix or `?token=` remediation (FR MDN also omits the guidance).
**Honest post-implementation claim:** "server-side exact-match Origin guard verified for allowed/disallowed/null/absent cases; browser enforcement, proxy forwarding, query-token exposure, and ADR gate #2 remain separately evidenced items."
**NV labels:** NV-1 lang gap · NV-2 not access control · NV-3 absent-Origin trade-off · NV-4 proxy stripping · NV-5 query token · NV-6 literal drift · NV-7 null/subdomains/list-header · NV-8 Tauri/CSP · NV-9 stale binary · NV-10 non-browser scope · NV-11 startup-only config.

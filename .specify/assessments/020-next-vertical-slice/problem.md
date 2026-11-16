# Problem: 020

After 019, search remains messages-only in the PG fallback path and task assignment produces no in-app feedback. On a cheap VPS without Meilisearch, users cannot discover core task/file entities via `/api/v1/search`, and assignees are not notified through existing notification primitives when work is assigned.

This leaves core collaboration loops incomplete for the Seed 80/20 scope (find and act on work) despite having the necessary Postgres data, notification schema/API, and proven 019 patterns (messages FTS fallback, best-effort reply notification, DB-gated contracts).

# Problem: 023

`GET /api/v1/search` currently prefers Meilisearch whenever `search_service` is configured, but no code path ever writes to the Meilisearch index (`index_task`/`index_file`/`index_message`/`delete_document` have zero call sites, `setup_index` is never invoked). A configured Meilisearch therefore returns empty (or stale) results while appearing healthy; 022's fail-open only covers query errors, not successful-but-empty responses. This is a correctness risk that conflicts with Flora's Seed profile (Flora + PostgreSQL only) and debloat goal.

Additionally, the PG FTS generated columns tokenize hyphenated file/task names as a single token under the `simple` config, so common filename sub-token queries cannot match. Config defaults drift (`enabled=true` in code vs `false` in `.env.example`), and there is no explicit capability/degradation contract in the API response.

# Problem: 021

After 020, task assignment is still only partially correct: the task PATCH contract cannot express explicit unassign (`assignee_id` absent and `null` collapse), so `Some -> None` transitions are impossible and notification semantics for unassign cannot be represented. Assignment notifications also lack a simple guard against obvious duplicate unread signals on repeated effective transitions/retries.

On search, PG fallback now covers tasks/files but edge/contract behavior (type normalization, bounds, deterministic expectations) is not fully locked by tests, while Meilisearch mode has no wired mutation-indexing parity for tasks/files/messages — creating ambiguity about backend precedence and multi-entity correctness in optional deployments.

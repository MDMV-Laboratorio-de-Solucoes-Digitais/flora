-- Opt-in wildcard de recuperação utilizável uma única vez (correção D11).
CREATE TABLE IF NOT EXISTS bootstrap_wildcard_state (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    consumed_at TIMESTAMPTZ,
    consumed_user_id UUID
);

INSERT INTO bootstrap_wildcard_state (singleton)
VALUES (TRUE)
ON CONFLICT (singleton) DO NOTHING;

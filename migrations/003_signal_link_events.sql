-- Signal linking history (append-only)

CREATE TABLE signal_links (
    id              BIGSERIAL PRIMARY KEY,
    signal_id       VARCHAR(32) NOT NULL REFERENCES signals(id),
    issue_id        VARCHAR(32) NOT NULL REFERENCES issues(id),
    linked_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    linked_by       VARCHAR(100) NOT NULL,
    reason          TEXT,

    CONSTRAINT signal_link_unique UNIQUE (signal_id, linked_at)
);

CREATE INDEX idx_signal_links_signal ON signal_links(signal_id);
CREATE INDEX idx_signal_links_issue ON signal_links(issue_id);

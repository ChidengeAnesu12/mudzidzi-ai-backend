CREATE TABLE topics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- Stable string identifier matching the node IDs already used by
    -- the Flutter app's Knowledge Map mock data (e.g. 'algebra',
    -- 'linear_equations') — keeps frontend and backend in lockstep
    -- without the Flutter app needing to know database UUIDs.
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    -- One of the 6 broad O-Level categories used for mastery
    -- aggregation (Topic Mastery radar, dashboards). A topic node can
    -- be a sub-topic (e.g. 'linear_equations') while still rolling up
    -- into a broad category ('algebra') for reporting.
    category TEXT NOT NULL CHECK (
        category IN ('numbers', 'algebra', 'functions', 'geometry', 'trigonometry', 'statistics')
    ),
    description TEXT,
    display_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_topics_category ON topics(category);

CREATE TRIGGER topics_set_updated_at
    BEFORE UPDATE ON topics
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
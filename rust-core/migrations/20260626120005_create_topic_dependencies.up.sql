CREATE TABLE topic_dependencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    prerequisite_topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT topic_dependencies_no_self_reference CHECK (topic_id <> prerequisite_topic_id),
    CONSTRAINT topic_dependencies_unique_edge UNIQUE (topic_id, prerequisite_topic_id)
);

-- Graph traversal needs to walk both directions efficiently:
-- "what does this topic require?" (filter by topic_id) and
-- "what does this topic unlock?" (filter by prerequisite_topic_id,
-- used by root-cause analysis walking backwards from a weak topic).
CREATE INDEX idx_topic_dependencies_topic_id ON topic_dependencies(topic_id);
CREATE INDEX idx_topic_dependencies_prerequisite_topic_id ON topic_dependencies(prerequisite_topic_id);
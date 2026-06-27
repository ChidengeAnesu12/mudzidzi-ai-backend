CREATE TABLE recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    recommendation_type TEXT NOT NULL CHECK (recommendation_type IN ('revise', 'practice', 'challenge')),
    reason TEXT NOT NULL,
    priority INT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Primary access pattern: "give me this student's active
-- recommendations, in priority order" (Dashboard's "Next Up" list).
CREATE INDEX idx_recommendations_student_active_priority
    ON recommendations(student_id, is_active, priority);
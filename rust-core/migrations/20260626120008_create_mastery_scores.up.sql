CREATE TABLE mastery_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    -- Bayesian Knowledge Tracing's P(L) — probability the student has
    -- mastered this topic, in [0, 1].
    mastery_probability NUMERIC(5, 4) NOT NULL CHECK (mastery_probability >= 0 AND mastery_probability <= 1),
    -- Denormalized convenience column (mastery_probability * 100), so
    -- the Rust Student/Analytics services can SELECT this directly
    -- without re-deriving it for every Dashboard/Progress response.
    mastery_percent NUMERIC(5, 2) NOT NULL CHECK (mastery_percent >= 0 AND mastery_percent <= 100),
    questions_attempted INT NOT NULL DEFAULT 0,
    questions_correct INT NOT NULL DEFAULT 0,
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT mastery_scores_unique_student_topic UNIQUE (student_id, topic_id)
);

CREATE INDEX idx_mastery_scores_student_id ON mastery_scores(student_id);
-- Powers Teacher Dashboard's "Topic Performance (Class Average)" —
-- aggregating every student's mastery for a single topic.
CREATE INDEX idx_mastery_scores_topic_id ON mastery_scores(topic_id);
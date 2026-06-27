CREATE TABLE attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    -- Denormalized from questions.topic_id at insert time. The
    -- Bayesian Knowledge Tracing recompute (Python service) reads
    -- long per-student-per-topic sequences very frequently — this
    -- avoids a join back to `questions` on every single read.
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    submitted_answer TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    time_taken_seconds INT,
    hint_used BOOLEAN NOT NULL DEFAULT FALSE,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_attempts_student_id ON attempts(student_id);
CREATE INDEX idx_attempts_question_id ON attempts(question_id);
-- Primary access pattern for BKT: "give me this student's attempts on
-- this topic, in chronological order".
CREATE INDEX idx_attempts_student_topic_chronological ON attempts(student_id, topic_id, attempted_at);
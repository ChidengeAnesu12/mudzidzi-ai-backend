use chrono::NaiveDate;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct StudentProfileRecord {
    pub full_name: String,
    pub email: String,
    pub school_name: Option<String>,
    pub grade_level: Option<String>,
}

pub async fn find_profile_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<StudentProfileRecord>, sqlx::Error> {
    sqlx::query_as::<_, StudentProfileRecord>(
        r#"
        SELECT u.full_name, u.email, s.name AS school_name, st.grade_level
        FROM users u
        JOIN students st ON st.user_id = u.id
        LEFT JOIN schools s ON s.id = st.school_id
        WHERE u.id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_student_id_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>("SELECT id FROM students WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

#[derive(Debug, FromRow)]
pub struct TopicMasteryRecord {
    pub topic_slug: String,
    pub topic_title: String,
    pub category: String,
    pub mastery_percent: f64,
    pub questions_attempted: i32,
    pub questions_correct: i32,
}

/// NUMERIC columns are cast to FLOAT8 in SQL rather than decoded as a
/// decimal type — our sqlx build doesn't enable the rust_decimal/
/// bigdecimal feature, and f64 precision is more than sufficient for
/// displaying a percentage.
pub async fn find_mastery_for_student(pool: &PgPool, student_id: Uuid) -> Result<Vec<TopicMasteryRecord>, sqlx::Error> {
    sqlx::query_as::<_, TopicMasteryRecord>(
        r#"
        SELECT
            t.slug AS topic_slug,
            t.title AS topic_title,
            t.category AS category,
            ms.mastery_percent::float8 AS mastery_percent,
            ms.questions_attempted,
            ms.questions_correct
        FROM mastery_scores ms
        JOIN topics t ON t.id = ms.topic_id
        WHERE ms.student_id = $1
        ORDER BY t.display_order
        "#,
    )
    .bind(student_id)
    .fetch_all(pool)
    .await
}

pub async fn count_questions_attempted(pool: &PgPool, student_id: Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT count(*) FROM attempts WHERE student_id = $1")
        .bind(student_id)
        .fetch_one(pool)
        .await
}

pub async fn count_questions_solved(pool: &PgPool, student_id: Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT count(*) FROM attempts WHERE student_id = $1 AND is_correct = true")
        .bind(student_id)
        .fetch_one(pool)
        .await
}

pub async fn distinct_activity_dates(pool: &PgPool, student_id: Uuid) -> Result<Vec<NaiveDate>, sqlx::Error> {
    sqlx::query_scalar::<_, NaiveDate>(
        "SELECT DISTINCT attempted_at::date FROM attempts WHERE student_id = $1 ORDER BY attempted_at::date DESC",
    )
    .bind(student_id)
    .fetch_all(pool)
    .await
}

#[derive(Debug, FromRow)]
pub struct RecommendationRecord {
    pub topic_slug: String,
    pub topic_title: String,
    pub recommendation_type: String,
    pub reason: String,
    pub priority: i32,
}

pub async fn find_active_recommendations(pool: &PgPool, student_id: Uuid) -> Result<Vec<RecommendationRecord>, sqlx::Error> {
    sqlx::query_as::<_, RecommendationRecord>(
        r#"
        SELECT t.slug AS topic_slug, t.title AS topic_title, r.recommendation_type, r.reason, r.priority
        FROM recommendations r
        JOIN topics t ON t.id = r.topic_id
        WHERE r.student_id = $1 AND r.is_active = true
        ORDER BY r.priority ASC
        "#,
    )
    .bind(student_id)
    .fetch_all(pool)
    .await
}
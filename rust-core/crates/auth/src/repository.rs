use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
}

#[derive(Debug, FromRow)]
pub struct ProfileRecord {
    pub school_name: Option<String>,
    pub grade_level: Option<String>,
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        "SELECT id, email, password_hash, full_name, role FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        "SELECT id, email, password_hash, full_name, role FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

/// Loads the school name + grade level for a student, or just the
/// school name for a teacher (grade_level is None in that case). Used
/// to build the UserResponse returned to Flutter after login/register.
pub async fn find_profile(pool: &PgPool, user_id: Uuid, role: &str) -> Result<ProfileRecord, sqlx::Error> {
    if role == "teacher" {
        sqlx::query_as::<_, ProfileRecord>(
            r#"
            SELECT s.name AS school_name, NULL::TEXT AS grade_level
            FROM teachers t
            LEFT JOIN schools s ON s.id = t.school_id
            WHERE t.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    } else {
        sqlx::query_as::<_, ProfileRecord>(
            r#"
            SELECT s.name AS school_name, st.grade_level
            FROM students st
            LEFT JOIN schools s ON s.id = st.school_id
            WHERE st.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }
}

/// Inserts a new user + role-specific profile row in a single
/// transaction, so a partial failure (e.g. the profile insert fails)
/// never leaves an orphaned `users` row with no matching student or
/// teacher record.
pub async fn create_user_with_profile(
    pool: &PgPool,
    full_name: &str,
    email: &str,
    password_hash: &str,
    role: &str,
) -> Result<Uuid, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (full_name, email, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(full_name)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .fetch_one(&mut *tx)
    .await?;

    if role == "teacher" {
        sqlx::query("INSERT INTO teachers (user_id) VALUES ($1)")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query("INSERT INTO students (user_id) VALUES ($1)")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(user_id)
}

pub async fn touch_last_login(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET last_login_at = now() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
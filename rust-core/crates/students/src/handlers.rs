use axum::{extract::State, Json};
use chrono::Utc;
use common::{AppError, AppResult, AppState, CurrentUser};
use uuid::Uuid;

use crate::dto::{
    RecommendationResponse, StudentProfileResponse, StudentProgressResponse, TopicMasteryResponse,
};
use crate::repository;
use crate::streak::compute_streak;

/// Resolves the `students.id` row for the current authenticated user.
/// Rejects with 401 (not 404) if the account isn't a student account
/// at all — e.g. a teacher calling a student-only endpoint by mistake.
async fn require_student_id(state: &AppState, current_user: &CurrentUser) -> AppResult<Uuid> {
    if current_user.role != "student" {
        return Err(AppError::Unauthorized(
            "This endpoint is only available to student accounts.".to_string(),
        ));
    }

    repository::find_student_id_by_user_id(&state.db, current_user.user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Student profile not found.".to_string()))
}

pub async fn get_profile(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> AppResult<Json<StudentProfileResponse>> {
    let profile = repository::find_profile_by_user_id(&state.db, current_user.user_id)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Student profile not found.".to_string()))?;

    Ok(Json(StudentProfileResponse {
        full_name: profile.full_name,
        email: profile.email,
        school_name: profile.school_name,
        grade_level: profile.grade_level,
    }))
}

pub async fn get_mastery(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> AppResult<Json<Vec<TopicMasteryResponse>>> {
    let student_id = require_student_id(&state, &current_user).await?;

    let records = repository::find_mastery_for_student(&state.db, student_id)
        .await
        .map_err(AppError::Database)?;

    Ok(Json(records.into_iter().map(into_mastery_response).collect()))
}

pub async fn get_progress(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> AppResult<Json<StudentProgressResponse>> {
    let student_id = require_student_id(&state, &current_user).await?;

    let mastery_records = repository::find_mastery_for_student(&state.db, student_id)
        .await
        .map_err(AppError::Database)?;

    let questions_attempted = repository::count_questions_attempted(&state.db, student_id)
        .await
        .map_err(AppError::Database)?;

    let questions_solved = repository::count_questions_solved(&state.db, student_id)
        .await
        .map_err(AppError::Database)?;

    let activity_dates = repository::distinct_activity_dates(&state.db, student_id)
        .await
        .map_err(AppError::Database)?;

    let study_streak_days = compute_streak(&activity_dates, Utc::now().date_naive());

    let overall_mastery_percent = if mastery_records.is_empty() {
        0.0
    } else {
        mastery_records.iter().map(|r| r.mastery_percent).sum::<f64>() / mastery_records.len() as f64
    };

    let topic_mastery = mastery_records.into_iter().map(into_mastery_response).collect();

    Ok(Json(StudentProgressResponse {
        overall_mastery_percent,
        questions_attempted,
        questions_solved,
        study_streak_days,
        topic_mastery,
    }))
}

pub async fn get_recommendations(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> AppResult<Json<Vec<RecommendationResponse>>> {
    let student_id = require_student_id(&state, &current_user).await?;

    let records = repository::find_active_recommendations(&state.db, student_id)
        .await
        .map_err(AppError::Database)?;

    let response = records
        .into_iter()
        .map(|r| RecommendationResponse {
            topic_slug: r.topic_slug,
            topic_title: r.topic_title,
            recommendation_type: r.recommendation_type,
            reason: r.reason,
            priority: r.priority,
        })
        .collect();

    Ok(Json(response))
}

fn into_mastery_response(r: repository::TopicMasteryRecord) -> TopicMasteryResponse {
    TopicMasteryResponse {
        topic_slug: r.topic_slug,
        topic_title: r.topic_title,
        category: r.category,
        mastery_percent: r.mastery_percent,
        questions_attempted: r.questions_attempted,
        questions_correct: r.questions_correct,
    }
}
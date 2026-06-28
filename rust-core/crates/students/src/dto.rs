use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StudentProfileResponse {
    pub full_name: String,
    pub email: String,
    pub school_name: Option<String>,
    pub grade_level: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TopicMasteryResponse {
    pub topic_slug: String,
    pub topic_title: String,
    pub category: String,
    pub mastery_percent: f64,
    pub questions_attempted: i32,
    pub questions_correct: i32,
}

#[derive(Debug, Serialize)]
pub struct StudentProgressResponse {
    pub overall_mastery_percent: f64,
    pub questions_attempted: i64,
    pub questions_solved: i64,
    pub study_streak_days: i32,
    pub topic_mastery: Vec<TopicMasteryResponse>,
}

#[derive(Debug, Serialize)]
pub struct RecommendationResponse {
    pub topic_slug: String,
    pub topic_title: String,
    pub recommendation_type: String,
    pub reason: String,
    pub priority: i32,
}
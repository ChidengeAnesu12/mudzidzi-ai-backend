use common::AppError;

use crate::dto::{LoginRequest, RegisterRequest};

/// Manual, dependency-free validation, mirroring the exact rules
/// already enforced client-side in the Flutter app's
/// RegisterScreen/LoginScreen — so the backend never rejects something
/// the UI already let through.
pub fn validate_register(payload: &RegisterRequest) -> Result<(), AppError> {
    if payload.full_name.trim().is_empty() {
        return Err(AppError::Validation("Full name is required.".to_string()));
    }
    validate_email(&payload.email)?;
    if payload.password.len() < 6 {
        return Err(AppError::Validation("Password must be at least 6 characters.".to_string()));
    }
    if payload.role != "student" && payload.role != "teacher" {
        return Err(AppError::Validation("Role must be 'student' or 'teacher'.".to_string()));
    }
    Ok(())
}

pub fn validate_login(payload: &LoginRequest) -> Result<(), AppError> {
    validate_email(&payload.email)?;
    if payload.password.is_empty() {
        return Err(AppError::Validation("Password is required.".to_string()));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), AppError> {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation("Email is required.".to_string()));
    }
    if !trimmed.contains('@') || !trimmed.contains('.') {
        return Err(AppError::Validation("Enter a valid email address.".to_string()));
    }
    Ok(())
}
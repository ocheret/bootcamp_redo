use std::borrow::BorrowMut;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::*, AppState};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<Json<QuestionDetail>, handlers_inner::HandlerError> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_question`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    match handlers_inner::create_question(question, &*questions_dao).await {
        Ok(question_detail) => Ok(Json(question_detail)),
        Err(e) => Err(e),
    }
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<Json<Vec<QuestionDetail>>, handlers_inner::HandlerError> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    match handlers_inner::read_questions(&*questions_dao).await {
        Ok(questions) => Ok(Json(questions)),
        Err(e) => Err(e),
    }
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<(), handlers_inner::HandlerError> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_question`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    match handlers_inner::delete_question(question_uuid, &*questions_dao).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<Json<AnswerDetail>, handlers_inner::HandlerError> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    match handlers_inner::create_answer(answer, &*answers_dao).await {
        Ok(answer_detail) => Ok(Json(answer_detail)),
        Err(e) => Err(e),
    }
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<Json<Vec<AnswerDetail>>, handlers_inner::HandlerError> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    match handlers_inner::read_answers(question_uuid, &*answers_dao).await {
        Ok(answers) => Ok(Json(answers)),
        Err(e) => Err(e),
    }
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> Result<(), handlers_inner::HandlerError> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_answer`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    match handlers_inner::delete_answer(answer_uuid, &*answers_dao).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

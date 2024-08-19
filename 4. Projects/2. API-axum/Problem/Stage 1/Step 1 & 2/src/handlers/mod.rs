use std::vec;

use crate::models::*;
use axum::{response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    println!("Create Question: {:?}", question);
    Json(QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: "A title".to_string(),
        description: "A description".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    println!("Read Questions");
    Json(vec![QuestionDetail {
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_string(),
        title: "A title".to_string(),
        description: "A description".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    }])
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    println!("Delete Question: {:?}", question_uuid);
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above
pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    println!("Create Answer: {:?}", answer);
    Json(AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: "b068cd2f-edac-479e-98f1-c5f91008dcbd".to_string(),
        content: "An answer".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    })
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above
pub async fn read_answers(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    println!("Read Answers: {:?}", question_uuid);
    Json(vec![AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_string(),
        question_uuid: "b068cd2f-edac-479e-98f1-c5f91008dcbd".to_string(),
        content: "An answer".to_string(),
        created_at: "2022-12-31 18:44:08.287442".to_string(),
    }])
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {
    println!("Delete Answer: {:?}", answer_uuid);
}
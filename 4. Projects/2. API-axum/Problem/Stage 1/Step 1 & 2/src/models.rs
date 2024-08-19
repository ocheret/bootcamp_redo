use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    // TODO: add a public `title` field of type String
    pub title: String,
    // TODO: add a public `description` field of type String
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    // TODO: add a public `question_uuid` field of type String
    pub question_uuid: String,
    // TODO: add a public `title` field of type String
    pub title: String,
    // TODO: add a public `description` field of type String
    pub description: String,
    // TODO: add a public `created_at` field of type String
    pub created_at: String,
}

// TODO: create a struct called `QuestionId`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionId {
    pub question_uuid: String,
}

// ----------

// TODO: create a struct called `Answer`
//       derive the following traits: Serialize, Deserialize
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

// TODO: create a struct called `AnswerDetail`
//       derive the following traits: Serialize, Deserialize, Debug, Clone, PartialEq
//       add a public `answer_uuid` field of type String
//       add a public `question_uuid` field of type String
//       add a public `content` field of type String
//       add a public `created_at` field of type String
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

// TODO: create a struct called `AnswerId`
//       derive the following traits: Serialize, Deserialize
//       add a public `answer_uuid` field of type String
#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerId {
    pub answer_uuid: String,
}

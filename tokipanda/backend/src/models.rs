use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "parent_role", rename_all = "snake_case")]
pub enum ParentRole {
    Father,
    Mother,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "child_gender", rename_all = "snake_case")]
pub enum ChildGender {
    Boy,
    Girl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "robot_status", rename_all = "snake_case")]
pub enum RobotStatus {
    Disconnected,
    Sleeping,
    Teaching,
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "session_status", rename_all = "snake_case")]
pub enum SessionStatus {
    LessonMode,
    QuestionMode,
    ListeningMode,
    MotionMode,
    PlayMode,
    ConsoleMode,
    RepeatMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "lesson_status", rename_all = "snake_case")]
pub enum LessonStatus {
    Untouched,
    OnGoing,
    Completed,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Package {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub banner_url: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Subject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub banner_url: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Lesson {
    pub id: Uuid,
    pub package: Uuid, // Foreign Key
    pub subject: Uuid, // Foreign Key
    pub description: String,
    pub audio_path: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub lesson_id: Uuid, // Foreign Key
    pub question_text: String,
    pub audio_path: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Answer_Variant {
    pub id: Uuid,
    pub question_id: Uuid, // Foreign Key
    pub answer_text: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Answer_Variant_Sample {
    pub id: Uuid,
    pub answer_variant: Uuid, // Foreign Key
    pub audio_path_list: Vec<String>,
    pub featured_vector_list: Vec<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Parent {
    pub id: Uuid,
    pub full_name: String,
    pub full_name_bangla: String,
    pub role: ParentRole,
    pub secondarry_full_name: Option<String>,
    pub secondary_full_name_bangla: Option<String>,
    pub secondary_role: Option<ParentRole>,
    pub email: Option<String>,
    pub phone: String,
    pub address: String,
    pub robot_serial_number: Uuid,
    pub is_active: bool,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Child {
    pub id: Uuid,
    pub parent_id: Uuid, // Foreign Key - Parent's ID
    pub full_name: String,
    pub full_name_bangla: String,
    pub nickname: String,
    pub nickname_bangla: String,
    pub gender: ChildGender,
    pub birth_day: NaiveDate,
    pub age: i32,
    pub child_image_url: Option<String>,
    pub current_package: Uuid, // Foreign Key
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Robot {
    pub serial_number: Uuid,
    pub parent_id: Option<Uuid>, // Foreign Key
    pub child_id_list: Option<Uuid>, // Foreign Key
    pub status: RobotStatus,
    pub session_mode: SessionStatus,
}
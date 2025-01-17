use crate::entities::{Submission, SubmissionObject};
use chrono::{DateTime, Local};

/// A trait for defining requests about submissions to the database.
#[axum::async_trait]
pub trait Submissions {
    async fn find_submission(&self, problem_id: i32) -> Option<Submission>;
    async fn get_all_submissions(&self) -> Vec<SubmissionObject>;
    async fn user_submitted(&self, user_id: i32) -> Vec<SubmissionObject>;
    async fn store_submission<'a>(
        &self,
        user_id: i32,
        problem_id: i32,
        submit_time: DateTime<Local>,
        asm: &'a str,
        is_ce: bool,
    ) -> Option<i32>;
}

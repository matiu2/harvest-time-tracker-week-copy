use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntries {
    pub per_page: usize,
    pub total_pages: usize,
    pub total_entries: usize,
    pub next_page: Option<usize>,
    pub previous_page: Option<usize>,
    pub time_entries: Vec<TimeEntry>,
    pub page: usize,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub first: String,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub last: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: usize,
    pub spent_date: String,
    pub hours: f32,
    pub notes: Option<String>,
    pub user: User,
    pub client: Client,
    pub project: Project,
    pub task: Task,
    pub external_reference: Option<ExternalReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: usize,
    pub name: String,
    pub currency: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExternalReference {
    pub id: String,
    pub group_id: String,
    pub account_id: Option<String>,
    pub permalink: String,
    pub service: String,
    pub service_icon_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadEntry {
    // The ID of the user to associate with the time entry. Defaults to the currently authenticated user’s ID.
    pub user_id: Option<usize>,
    // The ID of the project to associate with the time entry.
    pub project_id: usize,
    // The ID of the task to associate with the time entry.
    pub task_id: usize,
    // The ISO 8601 formatted date the time entry was spent.
    pub spent_date: String,
    // The current amount of time tracked. If provided, the time entry will be created with the specified hours and is_running will be set to false. If not provided, hours will be set to 0.0 and is_running will be set to true.
    pub hours: f64,
    // Any notes to be associated with the time entry.
    pub notes: Option<String>,
    // An object containing the id, group_id, account_id, and permalink of the external reference.
    pub external_reference: Option<ExternalReference>,
}

impl From<TimeEntry> for UploadEntry {
    fn from(input: TimeEntry) -> Self {
        Self {
            user_id: Some(input.user.id),
            project_id: input.project.id,
            task_id: input.task.id,
            spent_date: input.spent_date,
            hours: input.hours as f64,
            notes: input.notes,
            external_reference: input.external_reference,
        }
    }
}

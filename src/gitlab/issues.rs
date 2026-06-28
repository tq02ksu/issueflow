use serde::{Deserialize, Serialize};

use super::client;
#[cfg(test)]
use super::codec::decode_json;

#[derive(Debug)]
pub struct CreateIssueInput {
    pub project_id: u64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedIssue {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitlabIssue {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub web_url: String,
    pub milestone: Option<MilestoneRef>,
    pub labels: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneRef {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: u64,
    pub iid: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub due_date: Option<String>,
    pub web_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueNote {
    pub id: u64,
    pub body: String,
    pub author_name: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
struct IssueNoteWire {
    id: u64,
    body: String,
    system: bool,
    created_at: String,
    author: IssueNoteAuthorWire,
}

#[derive(Debug, Deserialize)]
struct IssueNoteAuthorWire {
    name: Option<String>,
}

#[derive(Serialize)]
struct CreateIssueBody<'a> {
    title: &'a str,
    description: &'a str,
}

pub async fn list_issues(
    base_url: &str,
    access_token: &str,
    project_id: u64,
    state: &str,
) -> Result<Vec<GitlabIssue>, String> {
    let client = client::build_client(base_url, access_token)?;
    let state = issue_state_filter(state)?;
    let mut query = vec![
        ("order_by", "updated_at".to_string()),
        ("sort", "desc".to_string()),
    ];
    if let Some(state) = state {
        query.push(("state", state.to_string()));
    }

    client
        .get_paginated(&format!("projects/{project_id}/issues"), &query)
        .await
}

pub async fn get_issue(
    base_url: &str,
    access_token: &str,
    project_id: u64,
    issue_iid: u64,
) -> Result<GitlabIssue, String> {
    let client = client::build_client(base_url, access_token)?;
    client
        .get_json(&format!("projects/{project_id}/issues/{issue_iid}"))
        .await
}

pub async fn create_issue(
    base_url: &str,
    access_token: &str,
    input: CreateIssueInput,
) -> Result<CreatedIssue, String> {
    let client = client::build_client(base_url, access_token)?;
    let path = format!("projects/{}/issues", input.project_id);
    let body = CreateIssueBody {
        title: &input.title,
        description: &input.description,
    };

    client.post_json(&path, &body).await
}

pub async fn list_project_milestones(
    base_url: &str,
    access_token: &str,
    project_id: u64,
) -> Result<Vec<Milestone>, String> {
    let client = client::build_client(base_url, access_token)?;
    client
        .get_paginated(&format!("projects/{project_id}/milestones"), &[])
        .await
}

pub async fn list_issue_notes(
    base_url: &str,
    access_token: &str,
    project_id: u64,
    issue_iid: u64,
) -> Result<Vec<IssueNote>, String> {
    let client = client::build_client(base_url, access_token)?;
    let notes: Vec<IssueNoteWire> = client
        .get_paginated(
            &format!("projects/{project_id}/issues/{issue_iid}/notes"),
            &[
                ("order_by", "created_at".to_string()),
                ("sort", "asc".to_string()),
            ],
        )
        .await?;

    Ok(filter_issue_notes(notes))
}

fn issue_state_filter(state: &str) -> Result<Option<&'static str>, String> {
    match state {
        "" | "opened" => Ok(Some("opened")),
        "closed" => Ok(Some("closed")),
        "all" => Ok(None),
        other => Err(format!("unsupported gitlab issue state: {other}")),
    }
}

fn filter_issue_notes(notes: Vec<IssueNoteWire>) -> Vec<IssueNote> {
    notes
        .into_iter()
        .filter(|note| !note.system)
        .map(|note| IssueNote {
            id: note.id,
            body: note.body,
            author_name: note.author.name.unwrap_or_else(|| "unknown".to_string()),
            created_at: note.created_at,
        })
        .collect()
}

#[cfg(test)]
fn parse_issue_response(body: &[u8]) -> Result<GitlabIssue, String> {
    decode_json(body, "issue")
}

#[cfg(test)]
fn parse_issue_notes_response(body: &[u8]) -> Result<Vec<IssueNote>, String> {
    let notes: Vec<IssueNoteWire> = decode_json(body, "notes")?;
    Ok(filter_issue_notes(notes))
}

#[cfg(test)]
fn parse_milestones_response(body: &[u8]) -> Result<Vec<Milestone>, String> {
    decode_json(body, "milestones")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_state_filter_rejects_unknown_states() {
        let err = issue_state_filter("pending").unwrap_err();

        assert!(err.contains("unsupported gitlab issue state"));
    }

    #[test]
    fn parse_issue_response_reads_milestone_reference() {
        let body = br#"{
            "id": 10,
            "iid": 3,
            "project_id": 99,
            "title": "Agent-created issue",
            "description": "Body",
            "state": "opened",
            "web_url": "https://gitlab.example.com/group/project/-/issues/3",
            "labels": ["agent", "backend"],
            "created_at": "2026-06-28T10:00:00Z",
            "updated_at": "2026-06-28T10:05:00Z",
            "milestone": {
                "id": 7,
                "title": "M1"
            }
        }"#;

        let issue = parse_issue_response(body).expect("issue should parse");

        assert_eq!(issue.id, 10);
        assert_eq!(issue.iid, 3);
        assert_eq!(issue.project_id, 99);
        assert_eq!(issue.labels, vec!["agent", "backend"]);
        assert_eq!(issue.milestone.expect("milestone").title, "M1");
    }

    #[test]
    fn parse_issue_notes_response_skips_system_notes() {
        let body = br#"[
            {
                "id": 1,
                "body": "system message",
                "system": true,
                "created_at": "2026-06-28T10:00:00Z",
                "author": { "name": "System" }
            },
            {
                "id": 2,
                "body": "human note",
                "system": false,
                "created_at": "2026-06-28T10:01:00Z",
                "author": { "name": "Alice" }
            }
        ]"#;

        let notes = parse_issue_notes_response(body).expect("notes should parse");

        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].id, 2);
        assert_eq!(notes[0].author_name, "Alice");
        assert_eq!(notes[0].body, "human note");
    }

    #[test]
    fn parse_milestones_response_reads_due_dates() {
        let body = br#"[
            {
                "id": 1,
                "iid": 2,
                "title": "M1",
                "description": "desc",
                "state": "active",
                "due_date": "2026-07-01",
                "web_url": "https://gitlab.example.com/group/project/-/milestones/2"
            }
        ]"#;

        let milestones = parse_milestones_response(body).expect("milestones should parse");

        assert_eq!(milestones.len(), 1);
        assert_eq!(milestones[0].due_date.as_deref(), Some("2026-07-01"));
    }
}

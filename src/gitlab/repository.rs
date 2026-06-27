use reqwest::Client;

pub async fn get_repo_file(
    base_url: &str,
    token: &str,
    project_id: u64,
    file_path: &str,
    reference: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let client = Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v4/projects/{project_id}/repository/files/{file_path}?ref={reference}"
        ))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;
    resp.json().await
}

use reqwest::Client;

pub async fn list_wiki_pages(
    base_url: &str,
    token: &str,
    project_id: u64,
) -> Result<serde_json::Value, reqwest::Error> {
    let client = Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v4/projects/{project_id}/wikis?with_content=false&per_page=50"
        ))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;
    resp.json().await
}

pub async fn get_wiki_page(
    base_url: &str,
    token: &str,
    project_id: u64,
    slug: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let client = Client::new();
    let resp = client
        .get(format!(
            "{base_url}/api/v4/projects/{project_id}/wikis/{slug}?render_html=false"
        ))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;
    resp.json().await
}

use serde::{Deserialize, Serialize};

use super::client;
#[cfg(test)]
use super::codec::decode_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPage {
    pub title: String,
    pub slug: String,
    pub format: String,
    pub content: Option<String>,
}

pub async fn list_wiki_pages(
    base_url: &str,
    access_token: &str,
    project_id: u64,
) -> Result<Vec<WikiPage>, String> {
    let client = client::build_client(base_url, access_token)?;
    client
        .get_paginated(
            &format!("projects/{project_id}/wikis"),
            &[("with_content", "false".to_string())],
        )
        .await
}

pub async fn get_wiki_page(
    base_url: &str,
    access_token: &str,
    project_id: u64,
    slug: &str,
) -> Result<WikiPage, String> {
    let client = client::build_client(base_url, access_token)?;
    client
        .get_json_query(
            &format!("projects/{project_id}/wikis/{slug}"),
            &[("render_html", "false".to_string())],
        )
        .await
}

#[cfg(test)]
fn parse_wiki_pages_response(body: &[u8]) -> Result<Vec<WikiPage>, String> {
    decode_json(body, "wiki pages")
}

#[cfg(test)]
fn parse_wiki_page_response(body: &[u8]) -> Result<WikiPage, String> {
    decode_json(body, "wiki page")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wiki_pages_response_reads_titles() {
        let body = br#"[
            {
                "title": "Runbook",
                "slug": "runbook",
                "format": "markdown"
            }
        ]"#;

        let pages = parse_wiki_pages_response(body).expect("wiki pages should parse");

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title, "Runbook");
        assert_eq!(pages[0].slug, "runbook");
    }

    #[test]
    fn parse_wiki_page_response_reads_content() {
        let body = br##"{
            "title": "Runbook",
            "slug": "runbook",
            "format": "markdown",
            "content": "# Hello"
        }"##;

        let page = parse_wiki_page_response(body).expect("wiki page should parse");

        assert_eq!(page.title, "Runbook");
        assert_eq!(page.content.as_deref(), Some("# Hello"));
    }
}

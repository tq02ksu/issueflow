use serde::{Deserialize, Serialize};

use super::client;
#[cfg(test)]
use super::codec::decode_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryFile {
    pub file_path: String,
    pub blob_id: String,
    pub content: String,
    pub encoding: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub size: u64,
}

pub async fn get_repo_file(
    base_url: &str,
    access_token: &str,
    project_id: u64,
    file_path: &str,
    reference: &str,
) -> Result<RepositoryFile, String> {
    let client = client::build_client(base_url, access_token)?;
    let mut url = client.api_url_segments([
        "projects",
        &project_id.to_string(),
        "repository",
        "files",
        file_path,
    ])?;
    url.query_pairs_mut().append_pair("ref", reference);

    client.get_json_url(url).await
}

#[cfg(test)]
fn parse_repo_file_response(body: &[u8]) -> Result<RepositoryFile, String> {
    decode_json(body, "repository file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_file_response_reads_content() {
        let body = br#"{
            "file_path": "AGENTS.md",
            "blob_id": "abc123",
            "content": "VGVzdA==",
            "encoding": "base64",
            "ref": "main",
            "size": 4
        }"#;

        let file = parse_repo_file_response(body).expect("repo file should parse");

        assert_eq!(file.file_path, "AGENTS.md");
        assert_eq!(file.blob_id, "abc123");
        assert_eq!(file.ref_name, "main");
        assert_eq!(file.encoding, "base64");
    }
}

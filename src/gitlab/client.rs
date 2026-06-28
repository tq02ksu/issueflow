use reqwest::{Method, Response, Url};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone, Debug)]
pub struct GitlabClient {
    http: reqwest::Client,
    api_base: Url,
    access_token: String,
}

pub fn build_client(base_url: &str, access_token: &str) -> Result<GitlabClient, String> {
    let api_base = ParsedBaseUrl::parse(base_url)?.api_base;
    let http = reqwest::Client::builder()
        .build()
        .map_err(|e| format!("failed to build gitlab http client: {e}"))?;

    Ok(GitlabClient {
        http,
        api_base,
        access_token: access_token.to_string(),
    })
}

impl GitlabClient {
    pub async fn get_json<T>(&self, path: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let url = self.api_url(path)?;
        self.request_json(Method::GET, url, Option::<&()>::None)
            .await
    }

    pub async fn get_json_query<T>(&self, path: &str, query: &[(&str, String)]) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let mut url = self.api_url(path)?;
        url.query_pairs_mut().extend_pairs(query.iter().cloned());
        self.request_json(Method::GET, url, Option::<&()>::None)
            .await
    }

    pub async fn get_json_url<T>(&self, url: Url) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        self.request_json(Method::GET, url, Option::<&()>::None)
            .await
    }

    pub async fn post_json<T, B>(&self, path: &str, body: &B) -> Result<T, String>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = self.api_url(path)?;
        self.request_json(Method::POST, url, Some(body)).await
    }

    pub async fn get_paginated<T>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<Vec<T>, String>
    where
        T: DeserializeOwned,
    {
        let mut next_page = Some("1".to_string());
        let mut items = Vec::new();

        while let Some(page) = next_page.take() {
            let mut page_query = query.to_vec();
            page_query.push(("per_page", "50".to_string()));
            page_query.push(("page", page));

            let mut url = self.api_url(path)?;
            url.query_pairs_mut().extend_pairs(page_query);

            let response = self.send(Method::GET, url, Option::<&()>::None).await?;
            next_page = response
                .headers()
                .get("x-next-page")
                .and_then(|value| value.to_str().ok())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string);

            let mut page_items = response
                .json::<Vec<T>>()
                .await
                .map_err(|e| format!("failed to decode gitlab paginated response: {e}"))?;
            items.append(&mut page_items);
        }

        Ok(items)
    }

    pub fn api_url(&self, path: &str) -> Result<Url, String> {
        self.api_base
            .join(path)
            .map_err(|e| format!("failed to build gitlab api url for {path}: {e}"))
    }

    pub fn api_url_segments<'a>(
        &self,
        segments: impl IntoIterator<Item = &'a str>,
    ) -> Result<Url, String> {
        let mut url = self.api_base.clone();
        let mut path_segments = url
            .path_segments_mut()
            .map_err(|_| "gitlab api base url does not support path segments".to_string())?;
        path_segments.pop_if_empty();
        for segment in segments {
            path_segments.push(segment);
        }
        drop(path_segments);
        Ok(url)
    }

    async fn request_json<T, B>(
        &self,
        method: Method,
        url: Url,
        body: Option<&B>,
    ) -> Result<T, String>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let response = self.send(method, url, body).await?;
        response
            .json::<T>()
            .await
            .map_err(|e| format!("failed to decode gitlab response: {e}"))
    }

    async fn send<B>(&self, method: Method, url: Url, body: Option<&B>) -> Result<Response, String>
    where
        B: Serialize + ?Sized,
    {
        let mut request = self
            .http
            .request(method, url.clone())
            .bearer_auth(&self.access_token);

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("failed to call gitlab api at {url}: {e}"))?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable body>".to_string());
            Err(format!("gitlab api request failed with {status}: {body}"))
        }
    }
}

#[derive(Debug)]
struct ParsedBaseUrl {
    api_base: Url,
}

impl ParsedBaseUrl {
    fn parse(base_url: &str) -> Result<Self, String> {
        let url = Url::parse(base_url).map_err(|e| format!("invalid gitlab base url: {e}"))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(format!(
                "unsupported gitlab base url scheme: {}",
                url.scheme()
            ));
        }
        if url.host_str().is_none() {
            return Err("gitlab base url is missing a host".to_string());
        }
        if url.path() != "/" && !url.path().is_empty() {
            return Err("gitlab base url must not include a path".to_string());
        }

        let api_base = url
            .join("/api/v4/")
            .map_err(|e| format!("failed to build gitlab api root: {e}"))?;

        Ok(Self { api_base })
    }
}

#[cfg(test)]
mod tests {
    use super::{ParsedBaseUrl, build_client};

    #[test]
    fn parse_base_url_keeps_port_and_adds_api_root() {
        let parsed = ParsedBaseUrl::parse("http://gitlab.example.com:8080").unwrap();

        assert_eq!(
            parsed.api_base.as_str(),
            "http://gitlab.example.com:8080/api/v4/"
        );
    }

    #[test]
    fn parse_base_url_rejects_path() {
        let err = ParsedBaseUrl::parse("https://gitlab.example.com/root").unwrap_err();

        assert!(err.contains("must not include a path"));
    }

    #[test]
    fn api_url_segments_encodes_nested_file_paths() {
        let client = build_client("https://gitlab.example.com", "user-token").unwrap();
        let url = client
            .api_url_segments(["projects", "42", "repository", "files", "docs/CONFIG.md"])
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://gitlab.example.com/api/v4/projects/42/repository/files/docs%2FCONFIG.md"
        );
    }
}

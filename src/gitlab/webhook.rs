use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitlabWebhook {
    pub object_kind: String,
    #[serde(default)]
    pub object_attributes: WebhookAttributes,
}

#[derive(Debug, Default, Deserialize)]
pub struct WebhookAttributes {
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub noteable_type: String,
}

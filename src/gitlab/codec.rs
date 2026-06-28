use serde::de::DeserializeOwned;

pub fn decode_json<T>(body: &[u8], resource: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    serde_json::from_slice(body).map_err(|e| format!("failed to decode gitlab {resource}: {e}"))
}

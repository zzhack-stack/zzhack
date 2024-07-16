use anyhow::Result;
use axum::http::StatusCode;

pub trait ResponseResultExt<T> {
    fn into_response_result(self, status_code: StatusCode) -> Result<T, (StatusCode, String)>;
}

impl<T> ResponseResultExt<T> for Result<T> {
    fn into_response_result(self, status_code: StatusCode) -> Result<T, (StatusCode, String)> {
        self.map_err(|err| (status_code, err.to_string()))
    }
}

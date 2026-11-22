use axum::{
    Json,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use macros_core::validator::CheckMateValidator;
use serde::de::DeserializeOwned;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S, Body> for ValidatedJson<T>
where
    S: Send + Sync,
    // T: DeserializeOwned + CheckMateValidator,
    T: DeserializeOwned + CheckMateValidator + Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        // if let Err(errors) = payload.validate() {
        if let Err(errors) = payload.validate().await {
            let mut response = Json(errors).into_response();
            *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
            return Err(response);
        }

        Ok(ValidatedJson(payload))
    }
}

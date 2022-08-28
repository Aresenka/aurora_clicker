use std::fmt::{Display, Formatter};
use actix_web::{
    get,
    HttpResponse,
    error::ResponseError,
    web::{Json, Path},
    http::{header::ContentType, StatusCode}
};
use crate::{
    model::user::UserStats,
    repository::user::{get_user_stats_by_address},
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserAddress {
    address: String
}

#[derive(Debug)]
pub enum StatsError {
    StatsNotFound
}

impl Display for StatsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatsError::StatsNotFound => write!(f, "No stats found.")
        }
    }
}

impl ResponseError for StatsError {
    fn status_code(&self) -> StatusCode {
        match self {
            StatsError::StatsNotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[get("/stats/{address}")]
pub async fn stats(address: Path<UserAddress>) -> Result<Json<UserStats>, StatsError> {
    let user_stats: Option<UserStats> = get_user_stats_by_address(address.into_inner().address);

    match user_stats {
        Some(stats) => Ok(Json(stats)),
        None => Err(StatsError::StatsNotFound)
    }
}
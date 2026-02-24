#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct GetCurrentTrackResponse {
    pub name: Option<String>,
}

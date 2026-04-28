use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HealthcheckResponse {
    pub accepting_traffic: bool,
}

impl HealthcheckResponse {
    pub fn accepting_traffic() -> Self {
        Self {
            accepting_traffic: true,
        }
    }
}

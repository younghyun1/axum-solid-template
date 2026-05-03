use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HealthcheckResponse {
    pub accepting_traffic: bool,
}

impl HealthcheckResponse {
    /// Builds the default healthcheck response that indicates the process is ready.
    ///
    /// # Returns
    /// Returns the value produced by this function.
    pub fn accepting_traffic() -> Self {
        Self {
            accepting_traffic: true,
        }
    }
}

use crate::client::FredClient;
use crate::params::V2ReleaseObservationsParams;
use crate::Result;

impl FredClient {
    /// Get release observations (v2 bulk endpoint).
    ///
    /// Unlike v1 endpoints, this uses bearer-token auth and cursor pagination.
    /// The response is left as `serde_json::Value` for now; see the FRED v2
    /// docs for the `has_more`/`next_cursor`/`series` shape.
    pub async fn release_observations_v2(
        &self,
        id: i32,
        params: V2ReleaseObservationsParams,
    ) -> Result<serde_json::Value> {
        let req = self
            .request_v2("v2/release/observations")
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }
}

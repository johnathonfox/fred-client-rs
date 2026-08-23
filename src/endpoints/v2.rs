use crate::client::FredClient;
use crate::params::V2ReleaseObservationsParams;
use crate::types::V2ReleaseObservations;
use crate::Result;

impl FredClient {
    /// Get release observations (v2 bulk endpoint).
    ///
    /// Unlike v1 endpoints, this uses bearer-token auth and cursor pagination.
    /// Pass `next_cursor` from each response in the params for the next request
    /// until `has_more` is false.
    pub async fn release_observations_v2(
        &self,
        id: i32,
        params: V2ReleaseObservationsParams,
    ) -> Result<V2ReleaseObservations> {
        let req = self
            .request_v2("v2/release/observations")
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }
}

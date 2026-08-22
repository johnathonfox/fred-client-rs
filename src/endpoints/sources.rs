use crate::client::FredClient;
use crate::error::FredError;
use crate::params::QueryParams;
use crate::types::{Release, Releases, Source, Sources};
use crate::Result;

impl FredClient {
    /// Get all sources.
    pub async fn sources(&self, params: QueryParams) -> Result<Sources> {
        let req = self.request("sources")?.query(&params);
        self.execute(req).await
    }

    /// Get a source by ID.
    pub async fn source(&self, id: i32) -> Result<Source> {
        let req = self.request("source")?.query(&[("source_id", id)]);
        let sources: Sources = self.execute(req).await?;
        sources
            .items
            .into_iter()
            .next()
            .ok_or_else(|| FredError::Parse("empty sources response".to_string()))
    }

    /// Get releases for a source.
    pub async fn source_releases(&self, id: i32, params: QueryParams) -> Result<Vec<Release>> {
        let req = self
            .request("source/releases")?
            .query(&[("source_id", id)])
            .query(&params);
        let releases: Releases = self.execute(req).await?;
        Ok(releases.items)
    }
}

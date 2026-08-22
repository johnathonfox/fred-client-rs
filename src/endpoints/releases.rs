use crate::client::FredClient;
use crate::error::FredError;
use crate::params::QueryParams;
use crate::types::{Release, ReleaseDates, Releases};
use crate::Result;

impl FredClient {
    /// Get all releases.
    pub async fn releases(&self, params: QueryParams) -> Result<Releases> {
        let req = self.request("releases")?.query(&params);
        self.execute(req).await
    }

    /// Get release dates for all releases.
    pub async fn releases_dates(&self, params: QueryParams) -> Result<ReleaseDates> {
        let req = self.request("releases/dates")?.query(&params);
        self.execute(req).await
    }

    /// Get a release by ID.
    pub async fn release(&self, id: i32) -> Result<Release> {
        let req = self.request("release")?.query(&[("release_id", id)]);
        let releases: Releases = self.execute(req).await?;
        releases
            .items
            .into_iter()
            .next()
            .ok_or_else(|| FredError::Parse("empty releases response".to_string()))
    }

    /// Get dates for a release.
    pub async fn release_dates(&self, id: i32, params: QueryParams) -> Result<ReleaseDates> {
        let req = self
            .request("release/dates")?
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get series for a release.
    pub async fn release_series(
        &self,
        id: i32,
        params: QueryParams,
    ) -> Result<crate::types::SeriesList> {
        let req = self
            .request("release/series")?
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get sources for a release.
    pub async fn release_sources(
        &self,
        id: i32,
        params: QueryParams,
    ) -> Result<crate::types::Sources> {
        let req = self
            .request("release/sources")?
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get tags for a release.
    pub async fn release_tags(&self, id: i32, params: QueryParams) -> Result<crate::types::Tags> {
        let req = self
            .request("release/tags")?
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get related tags for a release.
    pub async fn release_related_tags(
        &self,
        id: i32,
        tag_names: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<crate::types::Tags> {
        let req = self
            .request("release/related_tags")?
            .query(&[
                ("release_id", id.to_string()),
                ("tag_names", tag_names.as_ref().to_string()),
            ])
            .query(&params);
        self.execute(req).await
    }

    /// Get release tables.
    pub async fn release_tables(&self, id: i32, params: QueryParams) -> Result<serde_json::Value> {
        let req = self
            .request("release/tables")?
            .query(&[("release_id", id)])
            .query(&params);
        self.execute(req).await
    }
}

use crate::client::FredClient;
use crate::error::FredError;
use crate::params::{ObservationParams, QueryParams, SearchParams};
use crate::types::{Categories, Observations, Release, Releases, Series, SeriesList, Tags};
use crate::Result;

impl FredClient {
    /// Get a series by ID.
    pub async fn series(&self, id: impl AsRef<str>) -> Result<Series> {
        let req = self.request("series")?.query(&[("series_id", id.as_ref())]);
        let series: SeriesList = self.execute(req).await?;
        series
            .items
            .into_iter()
            .next()
            .ok_or_else(|| FredError::Parse("empty seriess response".to_string()))
    }

    /// Get categories for a series.
    pub async fn series_categories(
        &self,
        id: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<Categories> {
        let req = self
            .request("series/categories")?
            .query(&[("series_id", id.as_ref())])
            .query(&params);
        self.execute(req).await
    }

    /// Get observations for a series.
    pub async fn series_observations(
        &self,
        id: impl AsRef<str>,
        params: ObservationParams,
    ) -> Result<Observations> {
        let req = self
            .request("series/observations")?
            .query(&[("series_id", id.as_ref())])
            .query(&params);
        self.execute(req).await
    }

    /// Get the release for a series.
    pub async fn series_release(
        &self,
        id: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<Release> {
        let req = self
            .request("series/release")?
            .query(&[("series_id", id.as_ref())])
            .query(&params);
        let releases: Releases = self.execute(req).await?;
        releases
            .items
            .into_iter()
            .next()
            .ok_or_else(|| FredError::Parse("empty releases response".to_string()))
    }

    /// Search for series.
    pub async fn series_search(
        &self,
        text: impl AsRef<str>,
        params: SearchParams,
    ) -> Result<SeriesList> {
        let req = self
            .request("series/search")?
            .query(&[("search_text", text.as_ref())])
            .query(&params);
        self.execute(req).await
    }

    /// Get search tags for series search.
    pub async fn series_search_tags(
        &self,
        text: impl AsRef<str>,
        params: SearchParams,
    ) -> Result<Tags> {
        let req = self
            .request("series/search/tags")?
            .query(&[("series_search_text", text.as_ref())])
            .query(&params);
        self.execute(req).await
    }

    /// Get related tags for series search.
    pub async fn series_search_related_tags(
        &self,
        text: impl AsRef<str>,
        tag_names: impl AsRef<str>,
        params: SearchParams,
    ) -> Result<Tags> {
        let req = self
            .request("series/search/related_tags")?
            .query(&[
                ("series_search_text", text.as_ref()),
                ("tag_names", tag_names.as_ref()),
            ])
            .query(&params);
        self.execute(req).await
    }

    /// Get tags for a series.
    pub async fn series_tags(&self, id: impl AsRef<str>, params: QueryParams) -> Result<Tags> {
        let req = self
            .request("series/tags")?
            .query(&[("series_id", id.as_ref())])
            .query(&params);
        self.execute(req).await
    }

    /// Get updated series.
    pub async fn series_updates(&self, params: QueryParams) -> Result<SeriesList> {
        let req = self.request("series/updates")?.query(&params);
        self.execute(req).await
    }

    /// Get vintage dates for a series.
    pub async fn series_vintage_dates(
        &self,
        id: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<serde_json::Value> {
        let req = self
            .request("series/vintagedates")?
            .query(&[("series_id", id.as_ref())])
            .query(&params);
        self.execute(req).await
    }
}

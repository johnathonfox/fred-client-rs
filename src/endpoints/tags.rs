use crate::client::FredClient;
use crate::params::QueryParams;
use crate::types::{SeriesList, Tags};
use crate::Result;

impl FredClient {
    /// Get all tags.
    pub async fn tags(&self, params: QueryParams) -> Result<Tags> {
        let req = self.request("tags")?.query(&params);
        self.execute(req).await
    }

    /// Get related tags.
    pub async fn related_tags(
        &self,
        tag_names: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<Tags> {
        let req = self
            .request("related_tags")?
            .query(&[("tag_names", tag_names.as_ref())])
            .query(&params);
        self.execute(req).await
    }

    /// Get series for tags.
    pub async fn tags_series(
        &self,
        tag_names: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<SeriesList> {
        let req = self
            .request("tags/series")?
            .query(&[("tag_names", tag_names.as_ref())])
            .query(&params);
        self.execute(req).await
    }
}

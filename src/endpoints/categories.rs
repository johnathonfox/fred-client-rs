use crate::client::FredClient;
use crate::error::FredError;
use crate::params::QueryParams;
use crate::types::{Categories, Category};
use crate::Result;

impl FredClient {
    /// Get a category by ID.
    ///
    /// # Example
    /// ```no_run
    /// # async fn example(client: &fred_client_rs::FredClient) -> Result<(), fred_client_rs::FredError> {
    /// let category = client.category(125).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn category(&self, id: i32) -> Result<Category> {
        let req = self.request("category")?.query(&[("category_id", id)]);
        let categories: Categories = self.execute(req).await?;
        categories
            .items
            .into_iter()
            .next()
            .ok_or_else(|| FredError::Parse("empty categories response".to_string()))
    }

    /// Get child categories of a category.
    pub async fn category_children(&self, id: i32, params: QueryParams) -> Result<Categories> {
        let req = self
            .request("category/children")?
            .query(&[("category_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get related categories of a category.
    pub async fn category_related(&self, id: i32, params: QueryParams) -> Result<Categories> {
        let req = self
            .request("category/related")?
            .query(&[("category_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get series in a category.
    pub async fn category_series(
        &self,
        id: i32,
        params: QueryParams,
    ) -> Result<crate::types::SeriesList> {
        let req = self
            .request("category/series")?
            .query(&[("category_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get tags for a category.
    pub async fn category_tags(&self, id: i32, params: QueryParams) -> Result<crate::types::Tags> {
        let req = self
            .request("category/tags")?
            .query(&[("category_id", id)])
            .query(&params);
        self.execute(req).await
    }

    /// Get related tags for a category.
    pub async fn category_related_tags(
        &self,
        id: i32,
        tag_names: impl AsRef<str>,
        params: QueryParams,
    ) -> Result<crate::types::Tags> {
        let req = self
            .request("category/related_tags")?
            .query(&[
                ("category_id", id.to_string()),
                ("tag_names", tag_names.as_ref().to_string()),
            ])
            .query(&params);
        self.execute(req).await
    }
}

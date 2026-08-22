use fred_client_rs::params::{
    ObservationParams, QueryParams, SearchParams, V2ReleaseObservationsParams,
};
use fred_client_rs::types::{OutputType, Units};
use fred_client_rs::{FredClient, FredError};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_mock_server() -> (MockServer, FredClient) {
    let server = MockServer::start().await;
    let client = FredClient::builder()
        .api_key("test_key_12345678901234567890123456789012")
        .base_url(server.uri())
        .build()
        .unwrap();
    (server, client)
}

#[tokio::test]
async fn test_get_series() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "seriess": [{
            "id": "UNRATE",
            "title": "Unemployment Rate",
            "observation_start": "1948-01-01",
            "observation_end": "2024-01-01",
            "frequency": "Monthly",
            "frequency_short": "M",
            "units": "Percent",
            "units_short": "%",
            "seasonal_adjustment": "Seasonally Adjusted",
            "seasonal_adjustment_short": "SA",
            "last_updated": "2024-02-02 07:41:02-06:00",
            "popularity": 100,
            "group_popularity": 100,
            "notes": "The unemployment rate..."
        }]
    });

    Mock::given(method("GET"))
        .and(path("/series"))
        .and(query_param("series_id", "UNRATE"))
        .and(query_param(
            "api_key",
            "test_key_12345678901234567890123456789012",
        ))
        .and(query_param("file_type", "json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let series = client.series("UNRATE").await.unwrap();
    assert_eq!(series.id, "UNRATE");
    assert_eq!(series.title, "Unemployment Rate");
}

#[tokio::test]
async fn test_get_observations() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "observations": [
            {
                "realtime_start": "2024-01-01",
                "realtime_end": "2024-01-01",
                "date": "2023-01-01",
                "value": "3.4"
            },
            {
                "realtime_start": "2024-01-01",
                "realtime_end": "2024-01-01",
                "date": "2023-02-01",
                "value": "3.6"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/series/observations"))
        .and(query_param("series_id", "UNRATE"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let obs = client
        .series_observations("UNRATE", ObservationParams::new())
        .await
        .unwrap();
    assert_eq!(obs.items.len(), 2);
    assert_eq!(obs.items[0].value, Some("3.4".to_string()));
}

#[tokio::test]
async fn test_api_error() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/series"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Bad Request"))
        .mount(&server)
        .await;

    let result = client.series("INVALID").await;
    assert!(matches!(result, Err(FredError::Api { code: 400, .. })));
}

#[tokio::test]
async fn test_missing_api_key() {
    let result = FredClient::builder().build();
    assert!(matches!(result, Err(FredError::MissingApiKey)));
}

#[tokio::test]
async fn test_search_series() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "seriess": [
            {
                "id": "GDP",
                "title": "Gross Domestic Product",
                "observation_start": "1947-01-01",
                "observation_end": "2024-01-01",
                "frequency": "Quarterly",
                "frequency_short": "Q",
                "units": "Billions of Dollars",
                "units_short": "Bil. of $",
                "seasonal_adjustment": "Seasonally Adjusted Annual Rate",
                "seasonal_adjustment_short": "SAAR",
                "last_updated": "2024-02-28 07:46:03-06:00",
                "popularity": 100,
                "group_popularity": 100
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/series/search"))
        .and(query_param("search_text", "gdp"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let results = client
        .series_search("gdp", SearchParams::new())
        .await
        .unwrap();
    assert_eq!(results.items.len(), 1);
    assert_eq!(results.items[0].id, "GDP");
}

#[tokio::test]
async fn test_get_category_envelope() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "categories": [{
            "id": 125,
            "name": "Trade Balance",
            "parent_id": 13
        }]
    });

    Mock::given(method("GET"))
        .and(path("/category"))
        .and(query_param("category_id", "125"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let category = client.category(125).await.unwrap();
    assert_eq!(category.id, 125);
    assert_eq!(category.name, "Trade Balance");
}

#[tokio::test]
async fn test_source_releases_envelope() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "releases": [{
            "id": 53,
            "name": "Gross Domestic Product",
            "url": "https://fred.stlouisfed.org/release?rid=53"
        }]
    });

    Mock::given(method("GET"))
        .and(path("/source/releases"))
        .and(query_param("source_id", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let releases = client.source_releases(1, QueryParams::new()).await.unwrap();
    assert_eq!(releases.len(), 1);
    assert_eq!(releases[0].id, 53);
}

#[tokio::test]
async fn test_observation_params_serialization() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({ "observations": [] });

    Mock::given(method("GET"))
        .and(path("/series/observations"))
        .and(query_param("series_id", "UNRATE"))
        .and(query_param("units", "pc1"))
        .and(query_param("output_type", "2"))
        .and(query_param("limit", "100000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let params = ObservationParams::new()
        .units(Units::Pc1)
        .output_type(OutputType::VintageDateAll)
        .base(QueryParams::new().limit(100000));
    let obs = client.series_observations("UNRATE", params).await.unwrap();
    assert_eq!(obs.items.len(), 0);
}

#[tokio::test]
async fn test_series_search_tags_uses_series_search_text() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({ "tags": [] });

    Mock::given(method("GET"))
        .and(path("/series/search/tags"))
        .and(query_param("series_search_text", "gdp"))
        .and(query_param("tag_search_text", "monetary"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let tags = client
        .series_search_tags("gdp", SearchParams::new().tag_search_text("monetary"))
        .await
        .unwrap();
    assert_eq!(tags.items.len(), 0);
}

#[tokio::test]
async fn test_v2_release_observations_uses_bearer_and_cursor() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "has_more": "false",
        "series": []
    });

    Mock::given(method("GET"))
        .and(path("/v2/release/observations"))
        .and(header(
            "authorization",
            "Bearer test_key_12345678901234567890123456789012",
        ))
        .and(query_param("format", "json"))
        .and(query_param("release_id", "53"))
        .and(query_param("next_cursor", "cursor-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;

    let value = client
        .release_observations_v2(
            53,
            V2ReleaseObservationsParams::new().next_cursor("cursor-1"),
        )
        .await
        .unwrap();
    assert_eq!(value["has_more"], "false");
}

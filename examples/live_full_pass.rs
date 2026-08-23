use chrono::NaiveDate;
use fred_client_rs::params::{
    ObservationParams, QueryParams, SearchParams, V2ReleaseObservationsParams,
};
use fred_client_rs::{FredClient, FredError};
use std::time::Duration;

fn sanitize(err: &FredError) -> String {
    match err {
        FredError::Request(_) => "request error".to_string(),
        FredError::Api { code, .. } => format!("api error status {code}"),
        FredError::Validation(_) => "validation error".to_string(),
        FredError::MissingApiKey => "missing api key".to_string(),
        FredError::Parse(message) => format!("parse error: {message}"),
        FredError::Url(_) => "url error".to_string(),
    }
}

async fn pause() {
    // FRED asks clients to stay under roughly 2 requests/second.
    tokio::time::sleep(Duration::from_millis(650)).await;
}

async fn report<T, F>(
    name: &str,
    result: fred_client_rs::Result<T>,
    failures: &mut usize,
    render: F,
) where
    F: FnOnce(&T) -> String,
{
    match result {
        Ok(value) => println!("ok: {name} ({})", render(&value)),
        Err(err) => {
            *failures += 1;
            eprintln!("fail: {name} ({})", sanitize(&err));
        }
    }
    pause().await;
}

#[tokio::main]
async fn main() {
    let api_key = match std::env::var("FRED_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            eprintln!("FRED_API_KEY is not set");
            std::process::exit(2);
        }
    };

    let client = match FredClient::builder().api_key(api_key).build() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("client build failed: {}", sanitize(&err));
            std::process::exit(1);
        }
    };

    let mut failures = 0;
    let one = QueryParams::new().limit(1);

    report("category", client.category(0).await, &mut failures, |v| {
        format!("name={:?}", v.name)
    })
    .await;
    report(
        "category_children",
        client.category_children(0, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "category_related",
        client.category_related(125, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "category_series",
        client.category_series(125, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "category_tags",
        client.category_tags(125, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "category_related_tags",
        client.category_related_tags(125, "usa", one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;

    report(
        "releases",
        client.releases(one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "releases_dates",
        client.releases_dates(one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report("release", client.release(53).await, &mut failures, |v| {
        format!("name={:?}", v.name)
    })
    .await;
    report(
        "release_dates",
        client.release_dates(53, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "release_series",
        client.release_series(53, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "release_sources",
        client.release_sources(53, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "release_tags",
        client.release_tags(53, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "release_related_tags",
        client.release_related_tags(53, "usa", one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "release_tables",
        client
            .release_tables(53, fred_client_rs::params::ReleaseTablesParams::new())
            .await,
        &mut failures,
        |v| format!("elements={}", v.elements.len()),
    )
    .await;

    report(
        "series",
        client.series("UNRATE").await,
        &mut failures,
        |v| format!("title={:?}", v.title),
    )
    .await;
    report(
        "series_categories",
        client.series_categories("UNRATE", one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_observations",
        client
            .series_observations(
                "UNRATE",
                ObservationParams::new()
                    .observation_start(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
                    .observation_end(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
                    .base(one.clone()),
            )
            .await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_release",
        client.series_release("UNRATE", one.clone()).await,
        &mut failures,
        |v| format!("name={:?}", v.name),
    )
    .await;
    report(
        "series_search",
        client
            .series_search("gdp", SearchParams::new().limit(1))
            .await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_search_tags",
        client
            .series_search_tags("gdp", SearchParams::new().limit(1))
            .await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_search_related_tags",
        client
            .series_search_related_tags("gdp", "usa", SearchParams::new().limit(1))
            .await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_tags",
        client.series_tags("UNRATE", one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_updates",
        client.series_updates(one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "series_vintage_dates",
        client.series_vintage_dates("UNRATE", one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;

    let sources_result = client.sources(one.clone()).await;
    let source_id = sources_result
        .as_ref()
        .ok()
        .and_then(|sources| sources.items.first())
        .map(|source| source.id)
        .unwrap_or(1);
    report("sources", sources_result, &mut failures, |v| {
        format!("count={}", v.items.len())
    })
    .await;
    report(
        "source",
        client.source(source_id).await,
        &mut failures,
        |v| format!("name={:?}", v.name),
    )
    .await;
    report(
        "source_releases",
        client.source_releases(source_id, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.len()),
    )
    .await;

    let tags_result = client.tags(one.clone()).await;
    let tag_name = tags_result
        .as_ref()
        .ok()
        .and_then(|tags| tags.items.first())
        .map(|tag| tag.name.clone())
        .unwrap_or_else(|| "usa".to_string());
    report("tags", tags_result, &mut failures, |v| {
        format!("count={}", v.items.len())
    })
    .await;
    report(
        "related_tags",
        client.related_tags(tag_name.clone(), one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;
    report(
        "tags_series",
        client.tags_series(tag_name, one.clone()).await,
        &mut failures,
        |v| format!("count={}", v.items.len()),
    )
    .await;

    report(
        "release_observations_v2",
        client
            .release_observations_v2(53, V2ReleaseObservationsParams::new().limit(1))
            .await,
        &mut failures,
        |v| format!("has_more={}", v.has_more),
    )
    .await;

    if failures == 0 {
        println!("full live API pass passed");
    } else {
        eprintln!("full live API pass failed: {failures} request(s) failed");
        std::process::exit(1);
    }
}

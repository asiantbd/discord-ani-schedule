use reqwest;
use serde_json::{json, Value};
use std::error::Error as StdError;

const ANILIST_API_URL: &str = "https://graphql.anilist.co/";

pub async fn fetch_anime_dummy() -> Result<String, Box<dyn StdError + Send + Sync>> {
    let client = reqwest::Client::new();

    let query = r#"
    query Page($seasonYear: Int, $season: MediaSeason, $page: Int, $perPage: Int, $sort: [MediaSort], $type: MediaType) {
        Page(page: $page, perPage: $perPage) {
        media(seasonYear: $seasonYear, season: $season, sort: $sort, type: $type) {
            title {
            romaji
            }
            airingSchedule {
            nodes {
                episode
                airingAt
            }
            }
        }
        }
    }
    "#;

    let variables = json!({
        "seasonYear": 2024,
        "season": "FALL",
        "perPage": 1,
        "sort": "SCORE_DESC",
        "type": "ANIME"
    });

    let body = json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .post(ANILIST_API_URL)
        .json(&body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(serde_json::to_string_pretty(
        &response["data"]["Page"]["media"],
    )?)
}

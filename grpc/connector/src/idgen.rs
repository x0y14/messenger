use std::env;
use reqwest;
use dotenv::dotenv;

pub async fn get_new_id() -> Result<String, reqwest::Error> {
    dotenv().ok();
    let idgen_url = env::var("IDGEN_URL").expect("failed to get IDGEN_URL");
    let id = reqwest::get(idgen_url + "/id").await?.text().await?;

    Ok(id)
}

pub async fn get_new_revision() -> Result<String, reqwest::Error> {
    dotenv().ok();
    let idgen_url = env::var("IDGEN_URL").expect("failed to get IDGEN_URL");
    let revision = reqwest::get(idgen_url + "/revision").await?.text().await?;

    Ok(revision)
}
pub async fn pick_new_id() -> Result<String, reqwest::Error> {
    let body = reqwest::get("http://idgen:3030/id")
        .await?
        .text()
        .await?;

    Ok(body)
}

pub async fn pick_new_revision() -> Result<i64, reqwest::Error> {
    let body = reqwest::get("http://idgen:3030/revision")
        .await?
        .text()
        .await?;

    let rev = body.parse::<i64>().unwrap();

    Ok(rev)
}
pub mod supervisor_service { tonic::include_proto!("supervisor_service"); }
pub mod types { tonic::include_proto!("types"); }


pub async fn pick_new_id() -> Result<String, reqwest::Error> {
    let body = reqwest::get("http://idgen:3030/id")
        .await?
        .text()
        .await?;

    Ok(body)
}
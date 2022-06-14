use warp::Filter;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "id-gen");
    let ping = warp::path("ping").map(|| "ping");

    let revision = warp::path("revision").map(|| "Hello, World!");
    let id = warp::path("id").map(|| "id");

    warp::serve(root
        .or(ping)
        .or(revision)
        .or(id))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
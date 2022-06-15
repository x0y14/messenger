use warp::Filter;

#[tokio::main]
async fn main() {
    let addr = 3030;

    println!("id-gen listening on: {}", addr);

    let root = warp::path::end().map(|| "id-gen");
    let ping = warp::path("ping").map(|| "ping");

    let revision = warp::path("revision").map(|| "Hello, World!");
    let id = warp::path("id").map(|| "id");

    warp::serve(root
        .or(ping)
        .or(revision)
        .or(id))
        .run(([0, 0, 0, 0], addr))
        .await;
}
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let routes = warp::get().and_then(|q: String| async move {
        Ok(format!("Hello, {}!", q))
    });

    let route = routes.with(warp::log("access log"));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

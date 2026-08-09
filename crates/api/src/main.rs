mod state;
use std::net::SocketAddr;
use api::{controllers::inject_routers, state::AppState};
use tower_http::{cors::CorsLayer};

#[tokio::main]
async fn main() {
    println!("⚙️  Initializing Clean Architecture...");

    let state = AppState::new().await;
    application::add_application().await;
    infrastructure::add_infrastructure().await;
    query_handler::add_query_handler().await;

    let app = inject_routers(state)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("🚀 Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

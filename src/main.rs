use axum:: {
    routing::{get},
    //extract::Path,
    http::StatusCode,
    //response::IntoResponse,Json,
    Router
};



use std::net::SocketAddr;
//use serde:: {Deserialize, Serialize};

//use serde_json::json;

use tower_http::trace::{self, TraceLayer};

use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};






#[tokio::main]
async fn main() {
    
    //tracing_subscriber::registry()
      //    .with(tracing_subscriber::fmt::layer())
        //.init();
    
  
   
    //tracing_subscriber::fmt::init();
    
      tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
                );
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn health() -> Result<&'static str, StatusCode> {
    Ok("UP")
}
use axum::{routing::get, routing::post, Router};

#[tokio::main]
async fn main() {
    // should be able to interact with stats and weapons, and make modifications to them
    // modifying them may need to use the ar calculator, so both types have access to the
    // calculator

    let server_addr = "127.0.0.1:5000";

    let app = Router::new()
        .route("/profile", get(elden_builder_app::get_profile))
        .route("/stats", get(elden_builder_app::get_statlist))
        .route("/optimize", post(elden_builder_app::get_optimized_statlist))
        .route("/reset", post(elden_builder_app::get_reset_statlist))
        .route("/weapon_data", post(elden_builder_app::provide_weapon_data))
        .route(
            "/change_starter_class",
            post(elden_builder_app::change_starter_class),
        );

    println!(
        "Hello, world! -- server is up and running at {}.",
        server_addr
    );

    axum::Server::bind(&server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use exp_leptos::app::{shell, App};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data/newsletter.db".to_string());

    if let Some(rest) = database_url.strip_prefix("sqlite:") {
        let path = rest.split('?').next().unwrap_or(rest);
        if path != ":memory:" {
            if let Some(parent) = std::path::Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).expect("create db parent dir");
                }
            }
        }
    }

    let connect_options = SqliteConnectOptions::from_str(&database_url)
        .expect("parse DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(connect_options)
        .await
        .expect("connect sqlite");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS newsletter_subscribers (
            email TEXT PRIMARY KEY NOT NULL,
            subscribed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await
    .expect("create table");

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}

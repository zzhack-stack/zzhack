use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;

use api::database::{connection::get_db_connection, map_posts_to_db::map_posts_to_db};
use api::get_api_routes;
use app::portal::{ServerApp, ServerAppProps};
use axum::body::StreamBody;
use axum::error_handling::HandleError;
use axum::extract::{Query, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use futures::stream::{self, StreamExt};
use hyper::server::Server;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use yew::platform::Runtime;

// We use jemalloc as it produces better performance.
#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long)]
    dir: PathBuf,
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> impl IntoResponse {
    let url = url.to_string();
    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    StreamBody::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

// Map posts to database and initialize tables
pub fn initialize_data() {
    // initialize_tables().unwrap();
}

#[cfg(debug_assertions)]
fn get_port() -> usize {
    site_config::get_site_config().server.dev_port
}

#[cfg(not(debug_assertions))]
fn get_port() -> usize {
    site_config::get_site_config().server.prod_port
}

#[tokio::main]
async fn main() {
    let exec = Executor::default();

    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {e}"),
        )
    };

    let conn = get_db_connection().await;

    map_posts_to_db(&conn).await.unwrap();

    let app = Router::new()
        .nest("/api", get_api_routes())
        .with_state(api::AppState { conn })
        .fallback_service(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    get(render)
                        .with_state((index_html_before.clone(), index_html_after.clone()))
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            handle_error,
        ));

    let port = get_port();
    println!("Listening on http://localhost:{port}/");

    Server::bind(&format!("127.0.0.1:{port}").parse().unwrap())
        .executor(exec)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use std::sync::Arc;

use dioxus::prelude::*;
use indoc::formatdoc;

pub const JS_ASSETS: Asset = asset!("/assets/js");
pub const FONT_ASSETS: Asset = asset!("/assets/font");

mod ace_editor;
// mod turso;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone)]
pub enum DatabaseState {
    Disconnected,
    Connected(Arc<libsql_client::Client>),
    Error(String),
}

#[derive(Clone)]
pub struct DatabaseConnection {
    pub connection: Signal<DatabaseState>,
}

#[component]
fn App() -> Element {
    // use turso::TursoClient;
    use ace_editor::AceEditor;

    let con = use_signal(|| DatabaseState::Disconnected);
    use_context_provider(|| DatabaseConnection { connection: con });

    let font_style = formatdoc!(
        r#"
        @font-face {{
            font-family: 'Caskaydia Cove';
            src: url('{FONT_ASSETS}/CaskaydiaCove-Regular.woff2') format('woff2'),
                url('{FONT_ASSETS}/CaskaydiaCove-Regular.woff') format('woff');
            font-weight: normal;
            font-style: normal;
            font-display: swap;
        }}
        "#
    );

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Style { "{font_style}" }
        Header {}
        AceEditor {}
    }
}

#[component]
pub fn Header() -> Element {
    tracing::info!("hello from hero");
    use_effect(|| {
        tracing::info!("Connecting to db");
        let mut dbcon = use_context::<DatabaseConnection>().connection;
        spawn(async move {
            let client = libsql_client::Client::from_config(
                libsql_client::Config {
                    url: url::Url::parse("libsql://ic10emu-ryex.aws-us-east-1.turso.io").expect("invalid database url"),
                    auth_token: Some("eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3NjA1OTEzNTcsImlkIjoiNjFmYjYwZjQtNWM2My00ZDU4LTgxYjktOWMwYTRjOWIyNDAyIiwicmlkIjoiNzUyMTEzOGYtNDA1MC00MGQ5LWI5N2UtYTk5MzlmZTE0YTZmIn0.erny1662FB27_aOZFYK8YOtZp6GaJvNeHUl0Edisa8QGtUWBDSFWkn6bQFE-_jPGokgPun9yl4P0giFrOUx2Bw".to_string()),
                },
            ).await;
            match client {
                Ok(client) => {
                    tracing::info!("db connected");
                    dbcon.set(DatabaseState::Connected(Arc::new(client)))
                }
                Err(err) => {
                    tracing::info!("db errored: {:?}", &err);
                    dbcon.set(DatabaseState::Error(err.to_string()))
                }
            }
        });
    });
    let db_signal = use_context::<DatabaseConnection>().connection;
    let db_state = use_memo(move || match db_signal() {
        DatabaseState::Disconnected => "Disconnected".to_string(),
        DatabaseState::Connected(_) => "Connected".to_string(),
        DatabaseState::Error(ref err) => err.clone(),
    });
    rsx! {
        div {
            id: "header",
            div { p { "Database State: {db_state} "  } }
        }
    }
}

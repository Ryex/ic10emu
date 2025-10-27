use std::sync::Arc;

use dioxus::prelude::*;
use indoc::formatdoc;

pub const JS_ASSETS: Asset = asset!("/assets/js");
pub const FONT_ASSETS: Asset = asset!("/assets/font");

mod editor;
// mod turso;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone)]
pub struct BuildData {
    pub pkg_version: &'static str,
    pub git_branch: &'static str,
    pub git_commit: &'static str,
    pub git_commit_short: &'static str,
    pub git_dirty: bool,
    pub rustc_version: &'static str,
    pub rustc_version_semver: &'static str,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub build_data: Signal<BuildData>,
    pub ace_loaded: Signal<bool>,
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
    use editor::AceEditor;

    let build_data = use_signal(|| BuildData {
        pkg_version: env!("CARGO_PKG_VERSION"),
        git_branch: env!("GIT_BRANCH"),
        git_commit: env!("GIT_COMMIT"),
        git_commit_short: env!("GIT_COMMIT_SHORT"),
        git_dirty: env!("GIT_DIRTY") == "true",
        rustc_version: env!("RUSTC_VERSION"),
        rustc_version_semver: env!("RUSTC_VERSION_SEMVER"),
    });
    let ace_loaded = use_signal(|| false);
    use_context_provider(|| AppState {
        build_data,
        ace_loaded,
    });

    let con = use_signal(|| DatabaseState::Disconnected);
    use_context_provider(|| DatabaseConnection { connection: con });

    use_effect(|| {
        tracing::info!("Connecting to db");
        let mut dbcon = use_context::<DatabaseConnection>().connection;
        spawn(async move {
            let client = libsql_client::Client::from_config(libsql_client::Config {
                url: url::Url::parse(env!("SQLD_DB_URL")).expect("invalid database url"),
                auth_token: Some(env!("SQLD_AUTH_TOKEN").to_string()),
            })
            .await;
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

    let editor = use_signal(|| None);
    let session = use_signal(|| None);
    let ace_version: Signal<Option<String>> = use_signal(|| None);
    let div_id = use_signal(|| {
        let uid = uuid::Uuid::new_v4();
        format!("ace-editor-{}", uid.hyphenated().to_string())
    });
    use_context_provider(|| editor::EditorContext {
        div_id,
        ace_version,
        editor,
        session,
    });

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
    let build_data = use_context::<crate::AppState>().build_data;
    let pkg_version = use_signal(|| build_data.read().pkg_version);
    let commit_sha = use_signal(|| build_data.read().git_commit_short);
    let git_branch = use_signal(|| build_data.read().git_branch);
    let git_dirty = use_signal(|| {
        if build_data.read().git_dirty {
            " dirty"
        } else {
            ""
        }
    });
    let rustc_version = use_signal(|| build_data.read().rustc_version_semver);

    let db_signal = use_context::<DatabaseConnection>().connection;
    let db_state = use_memo(move || match db_signal() {
        DatabaseState::Disconnected => "Disconnected".to_string(),
        DatabaseState::Connected(_) => "Connected".to_string(),
        DatabaseState::Error(ref err) => err.clone(),
    });

    let ace_version_sig = use_context::<editor::EditorContext>().ace_version;
    let ace_version = use_memo(move || {
        let version = &*ace_version_sig.read();
        tracing::debug!("read ace version -> {version:?}");
        match version {
            Some(version) => version.to_owned(),
            None => "loading ...".to_string(),
        }
    });

    rsx! {
        nav { id: "nav", class: "relative ps-0 mb-0 flex flex-row flex-wrap items-start text-white",
            div {
                id: "app-state", class: "flex flex-col flex-wrap items-start",
                div { id: "rustc-data", p { "Rust Version: {rustc_version} " } }
                div { id: "git-data", p { "Version: {pkg_version}:{commit_sha}:{git_branch}{git_dirty}" } }
                div { id: "db-state", p { "Database: {db_state} " } }
                div { id: "ace-version", p { "Ace Version: {ace_version} " } }

            }
        }
    }
}

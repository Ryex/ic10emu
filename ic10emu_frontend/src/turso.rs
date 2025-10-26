use std::{collections::HashMap, sync::Arc};

use dioxus::{document::Script, prelude::*};
use js_sys::{Array, BigInt};
use wasm_bindgen::JsValue;

pub const LIBSQL_CLIENT_JS: Asset = asset!("/assets/static/js/turso.js");

#[wasm_bindgen]
extern "C" {
    pub type Client;
    pub type Row;
    #[wasm_bindgen(js_name = "createTursoClient", catch)]
    pub fn create_client(args: CreateClientArgs) -> Result<&'static Client, JsValue>;

    pub type ResultSet;

    #[wasm_bindgen(method, catch, js_name = "execute")]
    pub fn js_async_execute(this: &Client, sql: &str) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn rows(this: &ResultSet) -> Array<Row>;
    #[wasm_bindgen(method, getter)]
    pub fn columns(this: &ResultSet) -> Array<String>;
    #[wasm_bindgen(method, getter)]
    pub fn rows_affected(this: &ResultSet) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn last_insert_row_id(this: &ResultSet) -> Option<BigInt>;
}

impl Client {
    pub async fn execute(self, sql: &str) -> Result<ResultSet, JsValue> {
        let promise_as_jsvalue = self.js_async_execute(sql);
        let promist = js_sys::Promise::from(promise_as_js_value);
        let future = wasm_bindgen_futures::JsFuture::from(promise);

        future.await
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct CreateClientArgs {
    url: String,
    #[wasm_bindgen(js_name = "authToken")]
    auth_token: String,
    concurrency: Option<u32>,
    #[wasm_bindgen(js_name = "encurpttionKey")]
    encurpttion_key: Option<String>,
}

#[component]
pub fn TursoClient() -> Element {
    use_effect(|| {
        tracing::info!("Connecting to db");
        let mut dbcon = use_context::<crate::DatabaseConnection>().connection;
        let mut test_items = use_signal(|| Arc::new(HashMap::<String, String>::new()));
        spawn(async {
            let client = unsafe {
                create_client(CreateClientArgs {
                    url: "libsql://ic10emu-ryex.aws-us-east-1.turso.io".to_string(),
                    auth_token: "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJhIjoicnciLCJpYXQiOjE3NjA1OTEzNTcsImlkIjoiNjFmYjYwZjQtNWM2My00ZDU4LTgxYjktOWMwYTRjOWIyNDAyIiwicmlkIjoiNzUyMTEzOGYtNDA1MC00MGQ5LWI5N2UtYTk5MzlmZTE0YTZmIn0.erny1662FB27_aOZFYK8YOtZp6GaJvNeHUl0Edisa8QGtUWBDSFWkn6bQFE-_jPGokgPun9yl4P0giFrOUx2Bw".to_string(),
                    ..Default::default()
                })
            };
            match client {
                Ok(client) => {
                    tracing::info!("db connected");
                    dbcon.set(DatabaseState::Connected(Arc::new(client)));
                    let res = client.execute("SELECT * from testing").await;
                    match res {
                        Ok(rset) => {}
                        Err(err) => {
                            tracing::error!(err);
                        }
                    }
                }
                Err(err) => {
                    tracing::info!("db errored: {:?}", &err);
                    dbcon.set(DatabaseState::Error(err.to_string()));
                }
            }
        });
    });
    rsx! {
        Script {src: LIBSQL_CLIENT_JS}
        table { tr { th { "Key" } th { "Value" } }  }
    }
}

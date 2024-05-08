use futures::stream::TryStreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_lsp::{LspService, Server};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::stream::JsStream;

#[wasm_bindgen]
pub struct ServerConfig {
    into_server: js_sys::AsyncIterator,
    from_server: web_sys::WritableStream,
}

#[wasm_bindgen]
impl ServerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(into_server: js_sys::AsyncIterator, from_server: web_sys::WritableStream) -> Self {
        Self {
            into_server,
            from_server,
        }
    }
}

// NOTE: we don't use web_sys::ReadableStream for input here because on the
// browser side we need to use a ReadableByteStreamController to construct it
// and so far only Chromium-based browsers support that functionality.

// NOTE: input needs to be an AsyncIterator<Uint8Array, never, void> specifically
#[wasm_bindgen]
pub async fn serve(config: ServerConfig) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"server::serve".into());

    let ServerConfig {
        into_server,
        from_server,
    } = config;

    // tree_sitter::TreeSitter::init().await?;
    // let language = demo_lsp_language::language::javascript().await.unwrap();

    let input = JsStream::from(into_server);
    let input = input
        .map_ok(|value| {
            value
                .dyn_into::<js_sys::Uint8Array>()
                .expect("could not cast stream item to Uint8Array")
                .to_vec()
        })
        .map_err(|err| {
            web_sys::console::log_2(&"server::input Error: ".into(), &err);

            std::io::Error::from(std::io::ErrorKind::Other)
        })
        .into_async_read();

    let output = JsCast::unchecked_into::<wasm_streams::writable::sys::WritableStream>(from_server);
    let output = wasm_streams::WritableStream::from_raw(output);
    let output = output.try_into_async_write().map_err(|err| err.0)?;

    let (service, messages) = LspService::new(|client| ic10lsp_lib::server::Backend {
        client,
        files: Arc::new(RwLock::new(HashMap::new())),
        config: Arc::new(RwLock::new(ic10lsp_lib::server::Configuration::default())),
    });
    Server::new(input, output, messages).serve(service).await;

    web_sys::console::log_1(&"server::serve ic10lsp started".into());

    Ok(())
}

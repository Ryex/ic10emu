use dioxus::prelude::*;
use wasm_bindgen::JsValue;

// pub const ACE_EDITOR_JS: Asset = asset!("/assets/static/js/ace.js");
use crate::JS_ASSETS;

pub mod ace;

#[derive(Clone)]
pub struct EditorContext {
    pub div_id: Signal<String>,
    pub ace_version: Signal<Option<String>>,
    pub editor: Signal<Option<Rc<ace::Editor>>>,
    pub session: Signal<Option<Rc<ace::EditSession>>>,
}

#[component]
pub fn AceEditor() -> Element {
    let ace_editor_js: String = format!("{JS_ASSETS}/ace.js");
    let import_ace_js = ace_editor_js.clone();
    // ls lsp_worker_js: String = format!("{JS_ASSETS}/lspWorker.js");

    let ace_loaded = use_resource(move || {
        let import_ace_js = import_ace_js.clone();
        async move {
            let mut eval = document::eval(&format!(
                r#"
                    import("{import_ace_js}").then((module) => {{
                        window.ace = module.ace;
                        dioxus.send("ace_ready");
                    }})

                "#,
            ));

            let _res: String = eval.recv().await.unwrap();

            let mut context = use_context::<EditorContext>();

            let version = ace::ACE.version();
            context.ace_version.set(Some(version));

            let session = Rc::new(ace::create_edit_session(
                "#Some Test Text",
                Some("ace/mode/ic10".into()),
            ));

            use gloo_utils::format::JsValueSerdeExt;

            session.set_options(&ace::EditSessionOptions {
                first_line_number: Some(0),
                ..Default::default()
            });

            context.session.set(Some(session.clone()));
            true
        }
    });

    use_effect(move || match &*ace_loaded.read() {
        Some(_) => {
            let mut context = use_context::<EditorContext>();

            let div_id = context.div_id.read();

            tracing::info!("Starting editor on {}", &div_id);
            let editor: Result<ace::Editor, JsValue> = ace::edit(&div_id);
            match editor {
                Ok(editor) => {
                    tracing::info!("Ace editor created {:?}", &editor);

                    let editor = Arc::new(editor);
                    context.editor.set(Some(editor.clone()));
                }
                Err(js_error) => {
                    let err: js_sys::Error = js_error.into();
                    tracing::error!("Error creating editor: {:?}", &err);
                }
            }
        }
        None => {}
    });

    use_effect(|| {
        let context = use_context::<EditorContext>();

        let editor = &*context.editor.read();
        match editor {
            Some(editor) => {
                use gloo_utils::format::JsValueSerdeExt;

                editor.set_options(&ace::EditorOptions {
                    mode: Some("ace/mode/ic10".into()),
                    theme: Some("ace/theme/one_dark".into()),
                    font_family: Some("Caskaydia Cove".into()),
                    max_lines: Some(1000),
                    min_lines: Some(32),
                    first_line_number: Some(0),
                    print_margin: Some(ace::PrintMargin::Enable(true)),
                    print_margin_column: Some(128),
                    font_size: Some(ace::FontSize::Number(14.0)),
                    custom_scrollbar: Some(false),
                    wrap: Some(ace::LineWrap::Bool(false)),
                    enable_keyboard_accessibility: Some(true),
                    enable_mobile_menu: Some(true),
                    enable_basic_autocompletion: Some(true),
                    enable_live_autocompletion: Some(true),
                    enable_snippets: Some(true),
                    use_svg_gutter_icons: Some(true),
                    placeholder: Some("Your code goes here ...".into()),
                    ..Default::default()
                });

                editor.set_session(Some(context.session.read().as_ref().unwrap()));

                editor.resize();

                editor.on_change(Box::new(|delta: ace::AceDelta| {
                    tracing::debug!(
                        "Editor Change: {} start:{}:{} end:{}:{} lines: {:?}",
                        delta.action(),
                        delta.start().row(),
                        delta.start().column(),
                        delta.end().row(),
                        delta.end().column(),
                        delta.lines(),
                    );
                }));
            }
            None => {}
        }
    });

    let id = use_context::<EditorContext>().div_id;

    rsx! {
        document::Script { type: "module", src: "{ace_editor_js}" }
        div {
            class: "w-full h-full bg-[rgb(40,44,52)]",
            div { id: "{id}"  }
        }
    }
}

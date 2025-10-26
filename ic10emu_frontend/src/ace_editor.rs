use dioxus::{document::Script, prelude::*};
use std::sync::Arc;
use wasm_bindgen::{JsError, JsValue};

// pub const ACE_EDITOR_JS: Asset = asset!("/assets/static/js/ace.js");
use crate::JS_ASSETS;

pub mod ace {
    use serde::{Deserialize, Serialize};
    use serde_with::skip_serializing_none;
    use strum::EnumString;
    use wasm_bindgen::prelude::*;

    #[derive(Debug, Default, Deserialize, Serialize, EnumString)]
    #[strum(serialize_all = "lowercase")]
    pub enum CursorStyle {
        #[default]
        Ace,
        Slim,
        Smooth,
        Wide,
    }

    #[derive(Debug, Default, Deserialize, Serialize, EnumString)]
    #[strum(serialize_all = "lowercase")]
    pub enum FoldStyle {
        MarkBegin,
        #[default]
        MarkBeginEnd,
        Manual,
    }

    #[derive(Debug, Default, Deserialize, Serialize, EnumString)]
    #[strum(serialize_all = "lowercase")]
    pub enum NewLineMode {
        #[default]
        Auto,
        Unix,
        Windows,
    }

    #[derive(Debug, Default, Deserialize, Serialize, EnumString)]
    #[strum(serialize_all = "camelCase")]
    pub enum SelectionStyle {
        #[default]
        Line,
        Text,
        FullLine,
        ScreenLine,
    }

    #[derive(Debug, Default, Deserialize, Serialize, EnumString)]
    #[strum(serialize_all = "camelCase")]
    pub enum WrapMethod {
        #[default]
        Code,
        Text,
        Auto,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum FontSize {
        String(String),
        Number(f32),
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum PrintMargin {
        Enable(bool),
        Number(u32),
    }

    #[skip_serializing_none]
    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EditorOptions {
        pub animated_scroll: Option<bool>,
        pub auto_scroll_editor_into_view: Option<bool>,
        pub behaviours_enabled: Option<bool>,
        pub copy_with_empty_selection: Option<bool>,
        pub cursor_style: Option<CursorStyle>,
        pub custom_scrollbar: Option<bool>,
        pub display_indent_guides: Option<bool>,
        pub drag_delay: Option<f32>,
        pub drag_enabled: Option<bool>,
        pub enable_auto_indent: Option<bool>,
        pub enable_basic_autocompletion: Option<bool>, // | Completer[];
        pub enable_code_lens: Option<bool>,
        pub enable_keyboard_accessibility: Option<bool>,
        pub enable_live_autocompletion: Option<bool>, // | Completer[];
        pub enable_mobile_menu: Option<bool>,
        pub enable_multiselect: Option<bool>,
        pub enable_snippets: Option<bool>,
        pub fade_fold_widgets: Option<bool>,
        pub first_line_number: Option<i32>,
        pub fixed_width_gutter: Option<bool>,
        pub focus_timeout: Option<f32>,
        pub fold_style: Option<FoldStyle>,
        pub font_family: Option<String>,
        pub font_size: Option<FontSize>,
        pub h_scroll_bar_always_visible: Option<bool>,
        pub has_css_transforms: Option<bool>,
        pub highlight_active_line: Option<bool>,
        pub highlight_gutter_line: Option<bool>,
        pub highlight_indent_guides: Option<bool>,
        pub highlight_selected_word: Option<bool>,
        pub indented_soft_wrap: Option<bool>,
        pub keyboard_handler: Option<String>,
        pub live_autocompletion_delay: Option<f32>,
        pub live_autocompletion_threshold: Option<f32>,
        pub max_lines: Option<u32>,
        pub max_pixel_height: Option<u32>,
        pub merge_undo_deltas: Option<bool>, // | "always";
        pub min_lines: Option<u32>,
        pub mode: Option<String>,
        pub navigate_within_soft_tabs: Option<bool>,
        pub new_line_mode: Option<NewLineMode>,
        pub overwrite: Option<bool>,
        pub placeholder: Option<String>,
        pub print_margin: Option<PrintMargin>, // | number;
        pub print_margin_column: Option<u32>,
        pub read_only: Option<bool>,
        pub relative_line_numbers: Option<bool>,
        pub scroll_past_end: Option<f32>,
        pub scroll_speed: Option<f32>,
        pub selection_style: Option<SelectionStyle>,
        pub show_fold_widgets: Option<bool>,
        pub show_folded_annotations: Option<bool>,
        pub show_gutter: Option<bool>,
        pub show_invisibles: Option<bool>,
        pub show_line_numbers: Option<bool>,
        pub show_print_margin: Option<bool>,
        pub tab_size: Option<f32>,
        pub text_input_aria_label: Option<String>,
        pub theme: Option<String>,
        pub tooltip_follows_mouse: Option<bool>,
        pub use_resize_observer: Option<bool>,
        pub use_soft_tabs: Option<bool>,
        pub use_svg_gutter_icons: Option<bool>,
        pub use_worker: Option<bool>,
        pub v_scroll_bar_always_visible: Option<bool>,
        pub value: Option<String>,
        pub wrap: Option<bool>, // number | boolean | "off" | "free" | "printmargin";
        pub wrap_behaviours_enabled: Option<bool>,
        pub wrap_method: Option<WrapMethod>,
    }

    #[wasm_bindgen]
    extern "C" {
        pub static ace: Ace;
        pub type Ace;
        #[derive(Debug)]
        pub type Editor;
        #[derive(Debug)]
        pub type EditSession;

        pub type SyntaxMode;

        #[wasm_bindgen(method, getter)]
        pub fn version(this: &Ace) -> String;

        #[wasm_bindgen(js_namespace = ace, js_name = edit, catch)]
        pub fn edit(element: &str) -> Result<Editor, JsValue>;

        #[wasm_bindgen(js_namespace = ace, js_name = createEditSession)]
        pub fn create_edit_session(text: &str, mode: Option<&str>) -> EditSession;

        #[wasm_bindgen(method)]
        pub fn resize(this: &Editor);

        #[wasm_bindgen(method, catch, js_name = setOptions)]
        pub fn set_options(this: &Editor, opt_list: JsValue) -> Result<(), JsValue>;

        #[wasm_bindgen(method, js_name = setSession)]
        pub fn set_session(this: &Editor, session: Option<&EditSession>);

        #[wasm_bindgen(method, js_name = on)]
        pub fn bind_event(this: &Editor, name: &str, callback: JsValue, capturing: bool);

        pub type AcePoint;
        #[wasm_bindgen(method, getter)]
        pub fn column(this: &AcePoint) -> u32;
        #[wasm_bindgen(method, getter)]
        pub fn row(this: &AcePoint) -> u32;

        pub type AceDelta;

        #[wasm_bindgen(method, getter)]
        pub fn action(this: &AceDelta) -> String;
        #[wasm_bindgen(method, getter)]
        pub fn start(this: &AceDelta) -> AcePoint;
        #[wasm_bindgen(method, getter)]
        pub fn end(this: &AceDelta) -> AcePoint;
        #[wasm_bindgen(method, getter)]
        pub fn id(this: &AceDelta) -> Option<i64>;
        #[wasm_bindgen(method, getter)]
        pub fn lines(this: &AceDelta) -> Vec<String>;

    }

    impl Editor {
        pub fn on_change(&self, callback: Box<dyn FnMut(AceDelta)>) {
            self.bind_event("change", Closure::wrap(callback).into_js_value(), false);
        }
    }
}

#[derive(Clone)]
pub struct EditorContext {
    pub div_id: Signal<String>,
    pub ace_version: Signal<Option<String>>,
    pub editor: Signal<Option<Arc<ace::Editor>>>,
    pub session: Signal<Option<Arc<ace::EditSession>>>,
}

#[component]
pub fn AceEditor() -> Element {
    let id = use_signal(|| {
        let uid = uuid::Uuid::new_v4();
        format!("ace-editor-{}", uid.hyphenated().to_string())
    });
    tracing::info!("Creating editor {}", &id);

    let editor = use_signal(|| None);
    let session = use_signal(|| None);
    let ace_version: Signal<Option<String>> = use_signal(|| None);
    use_context_provider(|| EditorContext {
        div_id: id,
        ace_version,
        editor,
        session,
    });

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
            let session = Arc::new(ace::create_edit_session(
                "#Some Test Text",
                Some("ace/mode/ic10".into()),
            ));
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
            let version = ace::ace.version();
            context.ace_version.set(Some(version));
        }
        None => {}
    });

    use_effect(|| {
        let context = use_context::<EditorContext>();

        let editor = &*context.editor.read();
        match editor {
            Some(editor) => {
                use gloo_utils::format::JsValueSerdeExt;

                editor.set_options(
                    JsValue::from_serde(&ace::EditorOptions {
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
                        wrap: Some(false),
                        enable_keyboard_accessibility: Some(true),
                        enable_mobile_menu: Some(true),
                        enable_basic_autocompletion: Some(true),
                        enable_live_autocompletion: Some(true),
                        enable_snippets: Some(true),
                        placeholder: Some("Your code goes here ...".into()),
                        ..Default::default()
                    })
                    .expect("failed to serde editor options"),
                );

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

    rsx! {
        document::Script { type: "module", src: "{ace_editor_js}" }
        match &*ace_version.read() {
            Some(version) => rsx!{
                p { "Ace Version: {version}" }
            },
            None => rsx!{
                p { "loading ace ..."}
            }
        }
        div {
            class: "w-full h-full bg-[rgb(40,44,52)]",
            div { id: "{id}"  }
        }
    }
}

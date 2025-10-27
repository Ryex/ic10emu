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

#[derive(Debug, Default, Deserialize, Serialize, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum WrapType {
    #[default]
    Off,
    Free,
    Printmargin,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LineWrap {
    Number(u32),
    Bool(bool),
    Type(WrapMethod),
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
    pub print_margin: Option<PrintMargin>,
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
    pub wrap: Option<LineWrap>,
    pub wrap_behaviours_enabled: Option<bool>,
    pub wrap_method: Option<WrapMethod>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditSessionOptions {
    pub first_line_number: Option<u32>,
    pub fold_style: Option<FoldStyle>,
    pub indented_soft_wrap: Option<bool>,
    pub mode: Option<String>,
    pub navigate_within_soft_tabs: Option<bool>,
    pub new_line_mode: Option<NewLineMode>,
    pub overwrite: Option<bool>,
    pub tab_size: Option<u32>,
    pub use_soft_tabs: Option<bool>,
    pub use_worker: Option<bool>,
    pub wrap: Option<LineWrap>,
    pub wrap_method: Option<WrapMethod>,
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(thread_local_v2, js_name = ace)]
    pub static ACE: Ace;
    pub type Ace;
    #[derive(Debug)]
    pub type Editor;
    #[derive(Debug)]
    pub type EditSession;

    #[wasm_bindgen(method, catch, js_name = setOptions)]
    fn _set_options(this: &EditSession, opt_list: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn version(this: &Ace) -> String;

    #[wasm_bindgen(js_namespace = ace, js_name = edit, catch)]
    pub fn edit(element: &str) -> Result<Editor, JsValue>;

    #[wasm_bindgen(js_namespace = ace, js_name = createEditSession)]
    pub fn create_edit_session(text: &str, mode: Option<&str>) -> EditSession;

    #[wasm_bindgen(method)]
    pub fn resize(this: &Editor);

    #[wasm_bindgen(method, catch, js_name = setOptions)]
    fn _set_options(this: &Editor, opt_list: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = setSession)]
    pub fn set_session(this: &Editor, session: Option<&EditSession>);

    #[wasm_bindgen(method, js_name = on)]
    pub fn bind_event(this: &Editor, name: &str, callback: JsValue, capturing: bool);

    pub type SyntaxMode;

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

use gloo_utils::format::JsValueSerdeExt;

impl Editor {
    pub fn on_change(&self, callback: Box<dyn FnMut(AceDelta)>) {
        self.bind_event("change", Closure::wrap(callback).into_js_value(), false);
    }

    pub fn set_options(&self, opt_list: &EditorOptions) -> Result<(), JsValue> {
        let opt_list =
            JsValue::from_serde(opt_list).expect("failed to convert opt_list to JsValue");
        self._set_options(opt_list)
    }
}

impl EditSession {
    pub fn set_options(&self, opt_list: &EditSessionOptions) -> Result<(), JsValue> {
        let opt_list =
            JsValue::from_serde(opt_list).expect("failed to convert opt_list to JsValue");
        self._set_options(opt_list)
    }
}

use regex::{Captures, Regex};

pub fn strip_color(s: &str) -> String {
    let color_re = Regex::new(r"<color=.*?>|</color>").unwrap();
    color_re.replace_all(s, "").to_string()
}

#[allow(dead_code)]
pub fn color_to_html(s: &str) -> String {
    // not currently used
    // onig regex: r#"<color=(#?\w+)>((:?(?!<color=(?:#?\w+)>).)+?)</color>"#
    let color_re_start = Regex::new(r#"<color=(?<color>#?\w+)>"#).unwrap();
    let color_re_end = Regex::new("</color>").unwrap();
    let start_replaced = color_re_start.replace_all(s, |caps: &Captures| {
        format!(
            r#"<div style="color: {};">"#,
            caps.name("color").unwrap().as_str()
        )
    });
    let replaced = color_re_end.replace_all(&start_replaced, "</div>");
    replaced.to_string()
}

#[allow(dead_code)]
pub fn strip_link(s: &str) -> String {
    let link_re = Regex::new(r"<link=.*?>|</link>").unwrap();
    link_re.replace_all(s, "").to_string()
}

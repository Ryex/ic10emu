use onig::{Captures, Regex, RegexOptions, Syntax};

pub fn strip_color(s: &str) -> String {
    let color_regex = Regex::with_options(
        r#"<color=(#?\w+)>((:?(?!<color=(?:#?\w+)>).)+?)</color>"#,
        RegexOptions::REGEX_OPTION_MULTILINE | RegexOptions::REGEX_OPTION_CAPTURE_GROUP,
        Syntax::default(),
    )
    .unwrap();
    let mut new = s.to_owned();
    loop {
        new = color_regex.replace_all(&new, |caps: &Captures| caps.at(2).unwrap_or("").to_string());
        if !color_regex.is_match(&new) {
            break;
        }
    }
    new
}

#[allow(dead_code)]
pub fn color_to_heml(s: &str) -> String {
    let color_regex = Regex::with_options(
        r#"<color=(#?\w+)>((:?(?!<color=(?:#?\w+)>).)+?)</color>"#,
        RegexOptions::REGEX_OPTION_MULTILINE | RegexOptions::REGEX_OPTION_CAPTURE_GROUP,
        Syntax::default(),
    )
    .unwrap();
    let mut new = s.to_owned();
    loop {
        new = color_regex.replace_all(&new, |caps: &Captures| {
            format!(
                r#"<div style="color: {};">{}</div>"#,
                caps.at(1).unwrap_or(""),
                caps.at(2).unwrap_or("")
            )
        });
        if !color_regex.is_match(&new) {
            break;
        }
    }
    new
}

#[allow(dead_code)]
pub fn strip_link(s: &str) -> String {
    let link_regex = Regex::with_options(
        r#"<link=(\w+)>(.+?)</link>"#,
        RegexOptions::REGEX_OPTION_MULTILINE | RegexOptions::REGEX_OPTION_CAPTURE_GROUP,
        Syntax::default(),
    )
    .unwrap();
    let mut new = s.to_owned();
    loop {
        new = link_regex.replace_all(&new, |caps: &Captures| caps.at(2).unwrap_or("").to_string());
        if !link_regex.is_match(&new) {
            break;
        }
    }
    new
}

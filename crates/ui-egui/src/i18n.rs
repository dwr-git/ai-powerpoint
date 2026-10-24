//! The Russian interface (AI-PowerPoint).
//!
//! DeckCraft's interface text is English literals. AI-PowerPoint shows it in Russian through one
//! catalog, `crates/ui-egui/i18n/ru.tsv` (`English <TAB> Russian`, the craft-i18n row format without
//! the context column), looked up by the English text where it is drawn: the ribbon's buttons and
//! tabs, menus, dialogs, panes, the File view and the status bar. A string with no row shows in
//! English. Switch with `ui.language {value: ru|en}`; English is the default, so tests and the
//! desktop app read as upstream.
//!
//! [`tr`] looks a string up; the `tl!` macro is the same for a literal. Two-line ribbon labels
//! (`"New\nSlide"`) are looked up as written and then as one line.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

static RUSSIAN: AtomicBool = AtomicBool::new(false);

const RU_TSV: &str = include_str!("../i18n/ru.tsv");

/// Show the interface in Russian (`true`) or English.
pub fn set_russian(on: bool) {
    RUSSIAN.store(on, Ordering::Relaxed);
    // Empty placeholders prompt in the interface language too.
    deckcraft_model::set_russian_prompts(on);
}

/// The language text without its own proofing language is taken to be in: Russian with the
/// Russian interface, else US English (DeckCraft's default).
pub fn default_language() -> &'static str {
    if russian() { "ru-RU" } else { "en-US" }
}

/// Whether the interface is in Russian.
pub fn russian() -> bool {
    RUSSIAN.load(Ordering::Relaxed)
}

/// `\n`, `\t` and `\\` inside a catalog column.
fn unescape(s: &str) -> std::borrow::Cow<'static, str> {
    if !s.contains('\\') {
        return std::borrow::Cow::Owned(s.to_string());
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    std::borrow::Cow::Owned(out)
}

/// The catalog, read once. Rows with an escape are unescaped into leaked strings: the catalog
/// lives for the whole run, and is read once.
fn catalog() -> &'static HashMap<&'static str, &'static str> {
    static CATALOG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    CATALOG.get_or_init(|| parse(RU_TSV))
}

fn parse(text: &'static str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut cols = line.split('\t');
        let (Some(en), Some(ru)) = (cols.next(), cols.next()) else { continue };
        if en.is_empty() || ru.trim().is_empty() {
            continue;
        }
        let key: &'static str = if en.contains('\\') { Box::leak(unescape(en).into_owned().into_boxed_str()) } else { en };
        let value: &'static str = if ru.contains('\\') { Box::leak(unescape(ru).into_owned().into_boxed_str()) } else { ru };
        map.entry(key).or_insert(value);
    }
    map
}

/// The Russian for `s` when the interface is in Russian and the catalog has it, else `s`.
pub fn tr(s: &str) -> &str {
    if !russian() {
        return s;
    }
    lookup(s).unwrap_or(s)
}

/// The catalog's Russian for `s`, whatever the interface language: as written, without a
/// trailing ellipsis (kept on the translation), and a two-line label as one line.
pub fn lookup(s: &str) -> Option<&'static str> {
    let cat = catalog();
    if let Some(r) = cat.get(s) {
        return Some(r);
    }
    if s.contains('\n') {
        let one = s.replace('\n', " ");
        if let Some(r) = cat.get(one.as_str()) {
            return Some(r);
        }
    }
    if let Some(bare) = s.strip_suffix('…')
        && let Some(r) = cat.get(bare)
    {
        // Leaked once per distinct string: interface labels are a fixed, small set.
        return Some(ellipsis(r));
    }
    None
}

/// `r` with "…" appended, made once per string.
fn ellipsis(r: &'static str) -> &'static str {
    static MEMO: OnceLock<std::sync::Mutex<HashMap<&'static str, &'static str>>> = OnceLock::new();
    let memo = MEMO.get_or_init(Default::default);
    let mut m = memo.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(v) = m.get(r) {
        return v;
    }
    let v: &'static str = Box::leak(format!("{r}…").into_boxed_str());
    m.insert(r, v);
    v
}

/// [`tr`] of a template, then `{name}` placeholders filled from `args`.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = tr(template).to_string();
    for (k, v) in args {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// A literal translated where it is drawn: `tl!("Save")`.
#[macro_export]
macro_rules! tl {
    ($s:expr) => {
        $crate::i18n::tr($s)
    };
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[test]
    fn rows_parse_and_escapes_unescape() {
        let m = super::parse("# c\nSave\tСохранить\nNew\\nSlide\tСоздать\\nслайд\n\nbad line\nEmpty\t \n");
        assert_eq!(m.get("Save"), Some(&"Сохранить"));
        assert_eq!(m.get("New\nSlide"), Some(&"Создать\nслайд"));
        assert!(!m.contains_key("bad line") && !m.contains_key("Empty"));
    }

    #[test]
    fn english_is_the_default_and_unknown_text_stays() {
        assert_eq!(super::tr("Save"), "Save");
        assert_eq!(super::lookup("no such interface string at all"), None);
    }

    #[test]
    fn the_catalog_has_the_ribbon_tabs() {
        for tab in ["Home", "Insert", "Design", "Transitions", "Animations", "Slide Show", "Review", "View", "File"] {
            assert!(super::lookup(tab).is_some(), "{tab}");
        }
        // A trailing ellipsis is kept.
        assert!(super::lookup("Save As…").is_some_and(|s| s.ends_with('…')));
    }

    #[test]
    fn every_row_keeps_its_placeholders() {
        for (en, ru) in super::catalog() {
            for part in en.split('{').skip(1) {
                if let Some(name) = part.split('}').next() {
                    assert!(ru.contains(&format!("{{{name}}}")), "{en:?} → {ru:?} lost {{{name}}}");
                }
            }
        }
    }
}

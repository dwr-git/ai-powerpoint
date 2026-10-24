//! Design tokens (light, PowerPoint-like; and dark) and fonts for the UI.

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Visuals};
use serde::{Deserialize, Serialize};

/// The interface theme. AI-PowerPoint starts in `Brand` (navy chrome, the brand's blue-violet
/// accent, solid black menus and windows, the brand backdrop round the slide); `Light` and `Dark`
/// are the neumorphic panel palettes. Slides stay as the presentation draws them in every theme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Brightness {
    #[default]
    Light,
    Dark,
    Brand,
}

impl Brightness {
    /// In menu order.
    pub const ALL: [Brightness; 3] = [Brightness::Brand, Brightness::Light, Brightness::Dark];

    /// The value the `ui.theme` command takes and reports.
    pub fn code(self) -> &'static str {
        match self {
            Brightness::Light => "light",
            Brightness::Dark => "dark",
            Brightness::Brand => "brand",
        }
    }

    pub fn from_code(code: &str) -> Option<Brightness> {
        Brightness::ALL.into_iter().find(|b| b.code().eq_ignore_ascii_case(code.trim()))
    }

    /// The English label (translated where shown).
    pub fn label(self) -> &'static str {
        match self {
            Brightness::Light => "Light",
            Brightness::Dark => "Dark",
            Brightness::Brand => "Brand",
        }
    }

    /// Whether the interface is drawn with egui's dark visuals (Dark and Brand).
    pub fn is_dark(self) -> bool {
        !matches!(self, Brightness::Light)
    }
}

/// Whether the Brand palette is the one installed as egui's dark visuals ([`apply`]).
static BRAND: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The Brand theme is showing.
pub fn brand_on() -> bool {
    BRAND.load(std::sync::atomic::Ordering::Relaxed)
}

/// The colour the Brand backdrop reads as: the canvas token of the Brand theme.
pub const BRAND_CANVAS: Color32 = Color32::from_rgb(0x16, 0x18, 0x2C);

/// Every colour the chrome uses.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub dark: bool,
    /// Window background (title bar, tab row, side panes).
    pub chrome: Color32,
    /// The ribbon card and popovers.
    pub card: Color32,
    pub card_border: Color32,
    /// Text fields. They have no outline, so the fill alone must set them apart from the card.
    pub text_field: Color32,
    /// Canvas around the slide.
    pub canvas: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub border: Color32,
    pub separator: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub selected: Color32,
    /// DeckCraft's accent (tabs underline, selected thumbnail, primary buttons).
    pub accent: Color32,
    pub accent_text: Color32,
    pub contextual: Color32,
    pub selection: Color32,
    pub handle_fill: Color32,
    pub handle_stroke: Color32,
    pub guide: Color32,
    pub caret: Color32,
    pub text_selection: Color32,
    pub field_bg: Color32,
    /// Unchecked checkbox and radio boxes and slider rails, drawn without an outline, so they
    /// must stand apart from the card and the chrome.
    pub control: Color32,
    /// [`Tokens::control`] under the pointer and pressed: a step darker each in light mode.
    pub control_hover: Color32,
    pub control_pressed: Color32,
    pub shadow: Color32,
}

impl Tokens {
    /// Light: the neumorphic light panels (cool blue-grey chrome and cards, a blue accent).
    pub fn light() -> Self {
        let c = Color32::from_rgb;
        Tokens {
            dark: false,
            chrome: c(0xE4, 0xEC, 0xF6),
            card: c(0xE6, 0xEE, 0xF8),
            card_border: c(0xD0, 0xDB, 0xEA),
            text_field: c(0xD3, 0xDE, 0xEC),
            canvas: c(0xD6, 0xE0, 0xED),
            text: c(0x1E, 0x2C, 0x42),
            text_dim: c(0x34, 0x44, 0x5E),
            text_faint: c(0x4E, 0x5E, 0x78),
            border: c(0xB4, 0xC3, 0xD8),
            separator: c(0xD0, 0xDB, 0xEA),
            hover: c(0xD9, 0xE3, 0xF1),
            pressed: c(0xCC, 0xD9, 0xEB),
            selected: c(0xD3, 0xE2, 0xF8),
            accent: c(0x34, 0x80, 0xE2),
            accent_text: Color32::WHITE,
            contextual: c(0x16, 0x52, 0xA8),
            selection: c(0x5A, 0x6B, 0x85),
            handle_fill: Color32::WHITE,
            handle_stroke: c(0x34, 0x80, 0xE2),
            guide: c(0xD6, 0x3C, 0x7A),
            caret: Color32::BLACK,
            text_selection: Color32::from_rgba_unmultiplied(0x34, 0x80, 0xE2, 80),
            field_bg: c(0xF4, 0xF7, 0xFC),
            control: c(0xBA, 0xC8, 0xDC),
            control_hover: c(0xAE, 0xBD, 0xD3),
            control_pressed: c(0xA2, 0xB2, 0xCA),
            shadow: Color32::from_rgba_unmultiplied(0x46, 0x5A, 0x82, 56),
        }
    }
    /// Dark: the neumorphic dark panels (slate chrome and cards, a light blue accent with dark text
    /// on it).
    pub fn dark() -> Self {
        let c = Color32::from_rgb;
        Tokens {
            dark: true,
            chrome: c(0x26, 0x2D, 0x3A),
            card: c(0x28, 0x2F, 0x3D),
            card_border: c(0x36, 0x3F, 0x50),
            text_field: c(0x20, 0x26, 0x32),
            canvas: c(0x1B, 0x20, 0x2A),
            text: c(0xE4, 0xEB, 0xF6),
            text_dim: c(0xC4, 0xCF, 0xE0),
            text_faint: c(0xA0, 0xAD, 0xC2),
            border: c(0x4A, 0x56, 0x6C),
            separator: c(0x36, 0x3F, 0x50),
            hover: c(0x34, 0x3E, 0x50),
            pressed: c(0x3E, 0x4A, 0x60),
            selected: c(0x25, 0x37, 0x56),
            accent: c(0x5C, 0xA8, 0xFF),
            accent_text: c(0x0E, 0x1A, 0x2C),
            contextual: c(0x8E, 0xC6, 0xFF),
            selection: c(0xA0, 0xAD, 0xC2),
            handle_fill: Color32::WHITE,
            handle_stroke: c(0x5C, 0xA8, 0xFF),
            guide: c(0xFF, 0x77, 0xC6),
            caret: Color32::BLACK,
            text_selection: Color32::from_rgba_unmultiplied(0x5C, 0xA8, 0xFF, 90),
            field_bg: c(0x20, 0x26, 0x32),
            control: c(0x20, 0x26, 0x32),
            control_hover: c(0x34, 0x3E, 0x50),
            control_pressed: c(0x3E, 0x4A, 0x60),
            shadow: Color32::from_black_alpha(110),
        }
    }
    /// Brand: navy chrome, the brand's blue-violet accent (#667EEA), and the brand backdrop
    /// ([`paint_brand_backdrop`]) round the slide. Menus and windows are solid black ([`apply`]).
    pub fn brand() -> Self {
        let c = Color32::from_rgb;
        Tokens {
            dark: true,
            chrome: c(0x0C, 0x0D, 0x17),
            card: c(0x14, 0x15, 0x26),
            card_border: c(0x2A, 0x2D, 0x46),
            text_field: c(0x1F, 0x22, 0x38),
            canvas: BRAND_CANVAS,
            text: Color32::WHITE,
            text_dim: c(0xB7, 0xBB, 0xD8),
            text_faint: c(0x8C, 0x91, 0xB4),
            border: c(0x3A, 0x3E, 0x5E),
            separator: c(0x2A, 0x2D, 0x46),
            hover: c(0x22, 0x25, 0x3E),
            pressed: c(0x2D, 0x31, 0x52),
            selected: c(0x2A, 0x30, 0x66),
            accent: c(0x66, 0x7E, 0xEA),
            accent_text: c(0x0B, 0x0D, 0x1F),
            contextual: c(0xA9, 0xB4, 0xFF),
            selection: c(0xA9, 0xB4, 0xFF),
            handle_fill: Color32::WHITE,
            handle_stroke: c(0x66, 0x7E, 0xEA),
            guide: c(0xFF, 0x77, 0xC6),
            caret: Color32::BLACK,
            text_selection: Color32::from_rgba_unmultiplied(0x66, 0x7E, 0xEA, 100),
            field_bg: c(0x1F, 0x22, 0x38),
            control: c(0x2A, 0x2D, 0x46),
            control_hover: c(0x3A, 0x3E, 0x5E),
            control_pressed: c(0x4A, 0x4F, 0x75),
            shadow: Color32::BLACK,
        }
    }
    pub fn for_brightness(b: Brightness) -> Self {
        match b {
            Brightness::Light => Self::light(),
            Brightness::Dark => Self::dark(),
            Brightness::Brand => Self::brand(),
        }
    }
    pub fn get(ctx: &egui::Context) -> Self {
        if ctx.global_style().visuals.dark_mode {
            if brand_on() { Self::brand() } else { Self::dark() }
        } else {
            Self::light()
        }
    }
}

/// Apply tokens to egui's style.
pub fn apply(ctx: &egui::Context, t: &Tokens) {
    let brand = t.canvas == BRAND_CANVAS;
    BRAND.store(brand, std::sync::atomic::Ordering::Relaxed);
    let mut v = if t.dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.chrome;
    v.window_fill = t.card;
    v.extreme_bg_color = t.field_bg;
    v.text_edit_bg_color = Some(t.text_field);
    v.faint_bg_color = t.hover;
    v.window_stroke = Stroke::new(1.0, t.card_border);
    v.window_corner_radius = CornerRadius::same(10);
    v.menu_corner_radius = CornerRadius::same(8);
    v.selection.bg_fill = t.accent.gamma_multiply(0.85);
    v.selection.stroke = Stroke::new(1.0, t.accent_text);
    v.hyperlink_color = t.accent;
    v.override_text_color = Some(t.text);
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.corner_radius = CornerRadius::same(5);
    }
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.separator);
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.inactive.bg_fill = t.control;
    v.widgets.inactive.bg_stroke = Stroke::NONE;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_fill = t.control_hover;
    v.widgets.hovered.bg_stroke = Stroke::NONE;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.bg_fill = t.control_pressed;
    v.widgets.open.weak_bg_fill = t.pressed;
    v.window_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 18, spread: 0, color: t.shadow };
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 3], blur: 12, spread: 0, color: t.shadow };
    if brand {
        // Brand: menus, popups and windows are solid black blocks with a strong black shadow.
        v.window_fill = Color32::BLACK;
        v.window_stroke = Stroke::new(1.0, t.card_border);
        v.popup_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 32, spread: 0, color: Color32::BLACK };
        v.window_shadow = egui::epaint::Shadow { offset: [0, 10], blur: 36, spread: 2, color: Color32::BLACK };
    }
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(13.0));
        s.text_styles.insert(egui::TextStyle::Small, egui::FontId::proportional(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(18.0));
    });
}

/// UI fonts: Inter (from craft-fonts) when embedded, a Japanese UI face for CJK, then egui's.
pub fn install_fonts(ctx: &egui::Context) {
    install_fonts_with(ctx, &[]);
}

/// [`install_fonts`], with `extra` faces from the font database after every interface font: the
/// installed or embedded faces that draw text the interface faces lack (Hangul, Simplified
/// Chinese, Devanagari…, #101). See [`crate::uifonts`].
pub fn install_fonts_with(ctx: &egui::Context, extra: &[std::sync::Arc<deckcraft_fonts::FontFace>]) {
    ctx.set_fonts(font_definitions(extra));
}

/// The interface font definitions; see [`install_fonts_with`].
pub fn font_definitions(extra: &[std::sync::Arc<deckcraft_fonts::FontFace>]) -> FontDefinitions {
    let mut defs = FontDefinitions::default();
    let mut names = vec![];
    for (i, f) in deckcraft_fonts::CRAFT_FONTS.iter().enumerate() {
        let wanted = (f.family == "Inter" && (f.style == "Regular" || f.style == "SemiBold" || f.style == "Bold"))
            || (f.family == "BIZ UDPGothic" && f.style == "Regular")
            || (f.family == "Noto Sans Arabic");
        if !wanted {
            continue;
        }
        let name = format!("craft-{i}-{}-{}", f.family, f.style);
        defs.font_data.insert(name.clone(), std::sync::Arc::new(FontData::from_static(f.bytes)));
        names.push((f.family, f.style, name));
    }
    let regular: Vec<String> = names.iter().filter(|(fam, st, _)| !(*fam == "Inter" && *st != "Regular")).map(|(_, _, n)| n.clone()).collect();
    if let Some(list) = defs.families.get_mut(&FontFamily::Proportional) {
        for (k, n) in regular.iter().enumerate() {
            list.insert(k, n.clone());
        }
    }
    // A bold family for headings and selected tabs.
    let mut bold: Vec<String> = names.iter().filter(|(fam, st, _)| *fam == "Inter" && *st == "SemiBold").map(|(_, _, n)| n.clone()).collect();
    bold.extend(defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default());
    defs.families.insert(FontFamily::Name("bold".into()), bold);
    add_fallback_faces(&mut defs, extra);
    // The Japanese interface face (the first one when Inter isn't embedded) hints its en and em
    // dashes, bars thinner than a pixel, down to nothing, so they drew as gaps in interface text;
    // unhinted, they draw.
    for (name, d) in defs.font_data.iter_mut() {
        if name.contains("BIZ UDPGothic") {
            let mut data = (**d).clone();
            data.tweak.hinting = Some(false);
            *d = std::sync::Arc::new(data);
        }
    }
    defs
}

/// Add `faces` after every other font of each interface family (the last fallbacks).
pub fn add_fallback_faces(defs: &mut FontDefinitions, faces: &[std::sync::Arc<deckcraft_fonts::FontFace>]) {
    for f in faces {
        let name = format!("fallback {} {} {}", f.family, f.style, f.id());
        if defs.font_data.contains_key(&name) {
            continue;
        }
        let mut data = FontData::from_owned(f.data().to_vec());
        data.index = f.index();
        defs.font_data.insert(name.clone(), std::sync::Arc::new(data));
        for list in defs.families.values_mut() {
            list.push(name.clone());
        }
    }
}

/// The Brand theme's backdrop round the slide: the brand gradient (#0A0A0F → #1A1A2E → #16213E at
/// 135°), three soft glows, a 30-point grid of faint white lines and an inner shadow at the edges.
pub fn paint_brand_backdrop(painter: &egui::Painter, rect: egui::Rect) {
    use egui::epaint::{Mesh, Vertex, WHITE_UV};
    if !rect.is_positive() || !rect.is_finite() {
        return;
    }
    let vertex = |pos: egui::Pos2, color: Color32| Vertex { pos, uv: WHITE_UV, color };
    let quad = |m: &mut Mesh, corners: [(egui::Pos2, Color32); 4]| {
        let base = m.vertices.len() as u32;
        m.vertices.extend(corners.iter().map(|(p, c)| vertex(*p, *c)));
        m.indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    };
    let (a, mid, b) = (Color32::from_rgb(0x0A, 0x0A, 0x0F), Color32::from_rgb(0x1A, 0x1A, 0x2E), Color32::from_rgb(0x16, 0x21, 0x3E));
    let mut m = Mesh::default();
    quad(&mut m, [(rect.left_top(), a), (rect.right_top(), mid), (rect.right_bottom(), b), (rect.left_bottom(), mid)]);
    painter.add(egui::Shape::mesh(m));
    for (fx, fy, rgb, alpha) in [(0.2_f32, 0.8_f32, (120, 119, 198), 0.30_f32), (0.8, 0.2, (255, 119, 198), 0.30), (0.4, 0.4, (120, 219, 255), 0.20)] {
        let c = rect.min + egui::vec2(rect.width() * fx, rect.height() * fy);
        let far = egui::vec2(rect.width() * fx.max(1.0 - fx), rect.height() * fy.max(1.0 - fy)).length();
        let reach = far * 0.5;
        let inner = Color32::from_rgba_unmultiplied(rgb.0, rgb.1, rgb.2, (alpha * 255.0) as u8);
        let mut g = Mesh::default();
        g.vertices.push(vertex(c, inner));
        const STEPS: u32 = 64;
        for i in 0..=STEPS {
            let angle = std::f32::consts::TAU * i as f32 / STEPS as f32;
            g.vertices.push(vertex(c + egui::vec2(angle.cos(), angle.sin()) * reach, Color32::TRANSPARENT));
        }
        for i in 1..=STEPS {
            g.indices.extend([0, i, i + 1]);
        }
        painter.add(egui::Shape::mesh(g));
    }
    let line = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 13));
    let mut x = rect.left() + 0.5;
    while x < rect.right() {
        painter.vline(x, rect.y_range(), line);
        x += 30.0;
    }
    let mut y = rect.top() + 0.5;
    while y < rect.bottom() {
        painter.hline(rect.x_range(), y, line);
        y += 30.0;
    }
    let dark = |a: f32| Color32::from_rgba_unmultiplied(0, 0, 0, (a * 153.0) as u8);
    let mut sh = Mesh::default();
    let r = rect;
    for (i0, i1, a0, a1) in [(0.0_f32, 10.0_f32, 1.0_f32, 0.75_f32), (10.0, 45.0, 0.75, 0.0)] {
        quad(&mut sh, [(egui::pos2(r.left(), r.top() + i0), dark(a0)), (egui::pos2(r.right(), r.top() + i0), dark(a0)), (egui::pos2(r.right(), r.top() + i1), dark(a1)), (egui::pos2(r.left(), r.top() + i1), dark(a1))]);
        quad(&mut sh, [(egui::pos2(r.left(), r.bottom() - i1), dark(a1)), (egui::pos2(r.right(), r.bottom() - i1), dark(a1)), (egui::pos2(r.right(), r.bottom() - i0), dark(a0)), (egui::pos2(r.left(), r.bottom() - i0), dark(a0))]);
        quad(&mut sh, [(egui::pos2(r.left() + i0, r.top()), dark(a0)), (egui::pos2(r.left() + i1, r.top()), dark(a1)), (egui::pos2(r.left() + i1, r.bottom()), dark(a1)), (egui::pos2(r.left() + i0, r.bottom()), dark(a0))]);
        quad(&mut sh, [(egui::pos2(r.right() - i1, r.top()), dark(a1)), (egui::pos2(r.right() - i0, r.top()), dark(a0)), (egui::pos2(r.right() - i0, r.bottom()), dark(a0)), (egui::pos2(r.right() - i1, r.bottom()), dark(a1))]);
    }
    painter.add(egui::Shape::mesh(sh));
}

/// The area round the slide: the brand backdrop in the Brand theme, else `fill`.
pub fn paint_canvas_fill(painter: &egui::Painter, rect: egui::Rect, fill: Color32) {
    if fill == BRAND_CANVAS {
        paint_brand_backdrop(painter, rect);
    } else {
        painter.rect_filled(rect, 0.0, fill);
    }
}

pub fn bold(size: f32) -> egui::FontId {
    egui::FontId::new(size, FontFamily::Name("bold".into()))
}

pub fn font(size: f32) -> egui::FontId {
    egui::FontId::proportional(size)
}

pub fn to_color32(c: deckcraft_color::Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

#[cfg(test)]
mod tests {
    /// Each character of `text` laid out in the interface fonts at interface sizes, and whether it
    /// has ink (a glyph image).
    fn drawn(text: &str) -> Vec<(char, f32, bool)> {
        let ctx = egui::Context::default();
        ctx.set_fonts(super::font_definitions(&[]));
        let mut o = ctx.run_ui(Default::default(), |_| {});
        o.textures_delta.clear();
        let mut out = vec![];
        ctx.fonts_mut(|f| {
            for size in [11.0, 12.0, 13.0, 16.0] {
                for family in [egui::FontFamily::Proportional, egui::FontFamily::Name("bold".into())] {
                    let g = f.layout_no_wrap(text.to_string(), egui::FontId::new(size, family), egui::Color32::WHITE);
                    for gl in g.rows.iter().flat_map(|r| &r.glyphs) {
                        out.push((gl.chr, size, gl.uv_rect.size.x > 0.0 && gl.uv_rect.size.y > 0.0));
                    }
                }
            }
        });
        let mut o = ctx.run_ui(Default::default(), |_| {});
        o.textures_delta.clear();
        out
    }

    #[test]
    fn light_checkboxes_and_slider_rails_stand_out_from_their_background() {
        // An unchecked checkbox and a slider rail are filled with the inactive widget fill and
        // have no outline, so in light mode they vanished into the white dialog (#766).
        let ctx = egui::Context::default();
        super::apply(&ctx, &super::Tokens::light());
        let v = ctx.global_style().visuals.clone();
        assert_eq!(v.widgets.inactive.bg_stroke, egui::Stroke::NONE, "the test assumes no outline");
        let fill = v.widgets.inactive.bg_fill;
        for (name, bg) in [("dialog", v.window_fill), ("panel", v.panel_fill)] {
            let diff = [fill.r().abs_diff(bg.r()), fill.g().abs_diff(bg.g()), fill.b().abs_diff(bg.b())];
            assert!(diff.iter().all(|d| *d >= 24), "control fill {fill:?} is too close to the {name} background {bg:?}");
        }
        // Hovering and pressing darken the control a step each, as they do buttons.
        let (hovered, active) = (v.widgets.hovered.bg_fill, v.widgets.active.bg_fill);
        assert!(fill.r() > hovered.r() && hovered.r() > active.r(), "idle {fill:?}, hovered {hovered:?}, active {active:?} don't darken in turn");
    }

    #[test]
    fn thin_punctuation_is_drawn_in_interface_text() {
        // En and em dashes (thin bars) were hinted down to nothing in the Japanese interface face,
        // so notes and the outline showed gaps where they were.
        for (c, size, ink) in drawn("\u{2013}\u{2014}-_\u{2026}\u{2022}\u{00b7}\u{2018}\u{2019}\u{201c}\u{201d}=~") {
            assert!(ink, "{c:?} ({:04X}) draws nothing at {size} px", c as u32);
        }
    }
}

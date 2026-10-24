//! AI-PowerPoint: the Russian interface, in a process of its own (the language is process-wide,
//! and the library's tests read the English labels).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use deckcraft_engine::Session;
use deckcraft_ui_egui::{Services, SlideApp};
use egui_kittest::kittest::Queryable;
use serde_json::json;

fn harness(app: SlideApp) -> egui_kittest::Harness<'static, SlideApp> {
    egui_kittest::Harness::builder().with_size(egui::vec2(1400.0, 860.0)).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    )
}

#[test]
fn the_interface_switches_to_russian_and_every_tab_draws() {
    let mut app = SlideApp::new(Session::new(), Services::default());
    app.run("file.new", json!({})).unwrap();
    assert_eq!(app.run("ui.language", json!({"value": "ru"})).unwrap()["language"], "ru");
    assert_eq!(app.run("ui.theme", json!({"value": "brand"})).unwrap()["theme"], "brand");
    assert!(app.run("ui.theme", json!({"value": "sepia"})).is_err());
    let mut h = harness(app);
    h.run();
    // The ribbon's tabs and buttons, in Russian.
    h.get_by_label("Главная");
    h.get_by_label("Создать слайд");
    for tab in ["Insert", "Draw", "Design", "Transitions", "Animations", "Slide Show", "Record", "Review", "View"] {
        h.state_mut().run("view.tab", json!({"tab": tab})).unwrap();
        h.run();
    }
    // Empty placeholders prompt in Russian too.
    assert_eq!(deckcraft_model::PhType::Title.prompt(), "Щёлкните, чтобы добавить заголовок");
    // The File view, the light and dark themes.
    h.state_mut().run("app.backstage", json!({"page": "account"})).unwrap();
    h.run();
    for theme in ["light", "dark", "brand"] {
        h.state_mut().run("ui.theme", json!({"value": theme})).unwrap();
        h.run();
    }
    assert_eq!(h.state_mut().run("ui.language", json!({"value": "en"})).unwrap()["language"], "en");
    assert_eq!(deckcraft_model::PhType::Title.prompt(), "Click to add title");
}

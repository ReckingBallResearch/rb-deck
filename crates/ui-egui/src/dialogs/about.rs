use super::{Color32, CornerRadius, Rect, Sense, SlideApp, Tokens, Ui, pos2, theme, vec2};

/// Room the About credits need for their table; the About tab keeps its compact size.
pub(super) const CREDITS_SIZE: egui::Vec2 = vec2(820.0, 380.0);

/// The Contributors / Models body in a block of its own fixed size, below the tab row. Sizing the
/// dialog's `ui` instead (`set_min_size` / `set_max_height`) squeezed the tab row and the first
/// credits row together, so the text was garbled and the tabs stopped answering (#843).
pub(super) fn credits_block(ui: &mut Ui, models: bool) -> Rect {
    ui.allocate_ui(CREDITS_SIZE, |ui| {
        ui.set_min_size(CREDITS_SIZE);
        if models { crate::credits::models_ui(ui) } else { crate::credits::contributors_ui(ui) }
    })
    .response
    .rect
}

pub fn about(_app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.vertical_centered(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(64.0, 64.0), Sense::hover());
        paint_logo(ui.painter(), r);
        ui.label(egui::RichText::new("RB Deck").font(theme::bold(22.0)));
        ui.label(egui::RichText::new(version_line()).color(t.text_dim));
        ui.add_space(6.0);
        ui.label("Presentations for RB Home, by Recking Ball Research.");
        ui.label("Based on DeckCraft by the ArtCraft team. Not affiliated with Microsoft.");
        ui.add_space(6.0);
        ui.hyperlink_to("Source on GitHub", deckcraft_engine::links::GITHUB);
        ui.hyperlink_to("DeckCraft, the project RB Deck is based on", deckcraft_engine::links::BASED_ON);
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new("MIT OR Apache-2.0 · © 2026 Recking Ball Research, the ArtCraft Team and the DeckCraft contributors")
                .size(11.0)
                .color(t.text_faint),
        );
    });
}

/// "Version X.Y.Z" plus, for release builds, the short commit and build date. The version comes
/// from `[workspace.package] version` (the single source of truth, `cargo xtask version`); the
/// release workflow sets `DECKCRAFT_BUILD_SHA` / `DECKCRAFT_BUILD_DATE` at build time.
pub fn version_line() -> String {
    let mut s = format!("Version {}", env!("CARGO_PKG_VERSION"));
    let sha = option_env!("DECKCRAFT_BUILD_SHA").map(|s| s.get(..9).unwrap_or(s)).filter(|s| !s.is_empty());
    let date = option_env!("DECKCRAFT_BUILD_DATE").filter(|s| !s.is_empty());
    match (sha, date) {
        (Some(sha), Some(date)) => s.push_str(&format!(" ({sha}, {date})")),
        (Some(x), None) | (None, Some(x)) => s.push_str(&format!(" ({x})")),
        (None, None) => {}
    }
    s
}

/// RB Deck's mark, drawn in code: a slide card with a play triangle in our orange.
/// The RB Deck icon, drawn in code from the same shapes as `packaging/rb-icon.py` (a 512 grid).
pub fn paint_logo(p: &egui::Painter, r: Rect) {
    const NAVY: Color32 = Color32::from_rgb(0x1D, 0x2B, 0x4F);
    const SLIDE: Color32 = Color32::from_rgb(0xF7, 0xF8, 0xFA);
    const AMBER: Color32 = Color32::from_rgb(0xF5, 0xB3, 0x01);
    const TEAL: Color32 = Color32::from_rgb(0x2B, 0xB5, 0xA3);
    const SHAPES: [(f32, f32, f32, f32, f32, Color32); 8] = [
        (16.0, 16.0, 480.0, 480.0, 112.0, NAVY),
        (80.0, 112.0, 352.0, 232.0, 20.0, SLIDE),
        (112.0, 144.0, 150.0, 22.0, 6.0, AMBER),
        (128.0, 262.0, 52.0, 54.0, 6.0, TEAL),
        (206.0, 226.0, 52.0, 90.0, 6.0, TEAL),
        (284.0, 186.0, 52.0, 130.0, 6.0, TEAL),
        (236.0, 344.0, 40.0, 52.0, 0.0, SLIDE),
        (176.0, 392.0, 160.0, 24.0, 12.0, SLIDE),
    ];
    let k = r.width().min(r.height()) / 512.0;
    for (x, y, w, h, radius, colour) in SHAPES {
        let shape = Rect::from_min_size(pos2(r.min.x + x * k, r.min.y + y * k), vec2(w * k, h * k));
        p.rect_filled(shape, CornerRadius::same((radius * k).clamp(0.0, 255.0) as u8), colour);
    }
}

#[cfg(test)]
mod about_credits_tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::{CREDITS_SIZE, credits_block};
    use crate::dialogs::Dialog;
    use crate::{Services, SlideApp};

    fn tab_of(h: &Harness<'_, SlideApp>) -> String {
        let d = h.state().dialog.as_ref().expect("the About dialog is open");
        d.fields.get("tab").cloned().unwrap_or_else(|| "about".into())
    }

    /// The reported symptom (#843): in the real dialog the tabs stopped answering once Contributors
    /// or Models was open. Click through Contributors, Models and back to About.
    #[test]
    fn about_dialog_tabs_keep_responding() {
        let mut app = SlideApp::new(deckcraft_engine::Session::with_new(), Services::default());
        app.dialog = Some(Dialog::new("about"));
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_step_dt(1.0 / 60.0).build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        assert_eq!(tab_of(&h), "about");
        for label in ["Contributors", "Models", "About", "Models", "Contributors"] {
            h.get_by_label(label).click();
            h.run_steps(3);
            assert_eq!(tab_of(&h), label.to_lowercase(), "clicking {label} switches the tab");
        }
    }

    #[test]
    fn credits_get_their_own_block_below_the_tab_row() {
        for models in [false, true] {
            let ctx = egui::Context::default();
            let mut tabs = egui::Rect::NOTHING;
            let mut block = egui::Rect::NOTHING;
            let mut whole = egui::Rect::NOTHING;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                tabs = ui.label("About").rect;
                block = credits_block(ui, models);
                whole = ui.min_rect();
            });
            out.textures_delta.clear();
            assert_eq!(block.size(), CREDITS_SIZE, "models={models}");
            assert!(block.min.y >= tabs.max.y, "the credits start below the tab row (models={models})");
            assert!(whole.max.y >= block.max.y, "the dialog grows to hold the block (models={models})");
            assert!(whole.height() > CREDITS_SIZE.y, "the tab row is not squeezed into the credits height (models={models})");
        }
    }
}

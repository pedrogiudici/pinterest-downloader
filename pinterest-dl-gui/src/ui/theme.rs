use eframe::egui;

pub const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(13, 16, 24);
pub const CARD: egui::Color32 = egui::Color32::from_rgb(19, 23, 33);
pub const CARD_SOFT: egui::Color32 = egui::Color32::from_rgb(24, 29, 41);
pub const BORDER: egui::Color32 = egui::Color32::from_rgb(37, 43, 58);
pub const TEXT: egui::Color32 = egui::Color32::from_rgb(238, 241, 247);
pub const MUTED: egui::Color32 = egui::Color32::from_rgb(165, 171, 187);
pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(105, 76, 255);
pub const ACCENT_HOVER: egui::Color32 = egui::Color32::from_rgb(125, 91, 255);
pub const DANGER: egui::Color32 = egui::Color32::from_rgb(220, 53, 69);

pub fn configure_download_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = BACKGROUND;
    visuals.extreme_bg_color = BACKGROUND;
    visuals.override_text_color = Some(TEXT);
    visuals.widgets.noninteractive.bg_fill = CARD;
    visuals.widgets.inactive.bg_fill = CARD_SOFT;
    visuals.widgets.inactive.weak_bg_fill = CARD_SOFT;
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(30, 36, 52);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(38, 45, 64);
    visuals.selection.bg_fill = ACCENT;
    ctx.set_visuals(visuals);
}

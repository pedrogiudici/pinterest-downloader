use eframe::egui;

use crate::output::{ConsoleOutput, Output};

const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(13, 16, 24);
const CARD: egui::Color32 = egui::Color32::from_rgb(19, 23, 33);
const CARD_SOFT: egui::Color32 = egui::Color32::from_rgb(24, 29, 41);
const BORDER: egui::Color32 = egui::Color32::from_rgb(37, 43, 58);
const TEXT: egui::Color32 = egui::Color32::from_rgb(238, 241, 247);
const MUTED: egui::Color32 = egui::Color32::from_rgb(165, 171, 187);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(105, 76, 255);
const ACCENT_HOVER: egui::Color32 = egui::Color32::from_rgb(125, 91, 255);
const SUCCESS: egui::Color32 = egui::Color32::from_rgb(72, 205, 91);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextInputState {
    text: String,
    directory_path: String,
}

impl TextInputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn directory_path(&self) -> &str {
        &self.directory_path
    }

    pub fn set_directory_path(&mut self, directory_path: impl Into<String>) {
        self.directory_path = directory_path.into();
    }
}

#[derive(Debug)]
pub struct TextPrinterApp<O = ConsoleOutput> {
    state: TextInputState,
    output: O,
}

impl TextPrinterApp<ConsoleOutput> {
    pub fn new() -> Self {
        Self::with_output(ConsoleOutput)
    }
}

impl Default for TextPrinterApp<ConsoleOutput> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Output> TextPrinterApp<O> {
    pub fn with_output(output: O) -> Self {
        Self {
            state: TextInputState::new(),
            output,
        }
    }

    pub fn state(&self) -> &TextInputState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut TextInputState {
        &mut self.state
    }

    pub fn submit(&mut self) {
        self.output
            .handle_submission(self.state.text(), self.state.directory_path());
    }

    pub fn choose_directory_path(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.state.set_directory_path(path.display().to_string());
        }
    }

    pub fn into_output(self) -> O {
        self.output
    }

    fn destination_button_label(&self) -> String {
        match self.state.directory_path() {
            "" => "📁  Selecionar pasta".to_owned(),
            path => format!("📁  {}", compact_path(path, 24)),
        }
    }

    fn render_new_download_card(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(egui::CornerRadius::same(14))
            .inner_margin(egui::Margin::symmetric(36, 32))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⇩").size(34.0).color(ACCENT));
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Novo Download")
                                .size(26.0)
                                .strong()
                                .color(TEXT),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("Cole o link do Pinterest que deseja baixar")
                                .size(17.0)
                                .color(MUTED),
                        );
                    });
                });

                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    let input_width = (ui.available_width() - 424.0).max(260.0);
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(14, 17, 26))
                        .stroke(egui::Stroke::new(1.5, ACCENT))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(16, 10))
                        .show(ui, |ui| {
                            ui.set_width(input_width);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("🔗").size(22.0).color(MUTED));
                                ui.add_space(10.0);
                                let text_field = ui.add(
                                    egui::TextEdit::singleline(&mut self.state.text)
                                        .desired_width(input_width - 50.0)
                                        .hint_text("https://br.pinterest.com/pin/..."),
                                );

                                let enter_pressed = text_field.lost_focus()
                                    && ui.input(|input| input.key_pressed(egui::Key::Enter));
                                if enter_pressed {
                                    self.submit();
                                }
                            });
                        });

                    ui.add_space(16.0);

                    let folder_label = self.destination_button_label();
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(folder_label).size(16.0).color(TEXT),
                            )
                            .fill(CARD_SOFT)
                            .stroke(egui::Stroke::new(1.0, BORDER))
                            .corner_radius(egui::CornerRadius::same(8))
                            .min_size(egui::vec2(210.0, 60.0)),
                        )
                        .clicked()
                    {
                        self.choose_directory_path();
                    }

                    ui.add_space(16.0);

                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("⇩  Baixar")
                                    .size(17.0)
                                    .strong()
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(ACCENT)
                            .stroke(egui::Stroke::new(1.0, ACCENT_HOVER))
                            .corner_radius(egui::CornerRadius::same(8))
                            .min_size(egui::vec2(160.0, 60.0)),
                        )
                        .clicked()
                    {
                        self.submit();
                    }
                });
            });
    }
}

impl<O: Output> eframe::App for TextPrinterApp<O> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        configure_download_theme(ui.ctx());

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BACKGROUND))
            .show(ui, |ui| {
                ui.add_space(28.0);
                ui.horizontal(|ui| {
                    ui.add_space(28.0);
                    ui.label(egui::RichText::new("🦀").size(26.0));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Pinterest Downloader")
                            .size(20.0)
                            .color(TEXT),
                    );
                });

                ui.add_space(28.0);
                ui.horizontal(|ui| {
                    ui.add_space(28.0);
                    ui.vertical(|ui| {
                        ui.set_width((ui.available_width() - 28.0).max(0.0));
                        self.render_new_download_card(ui);
                    });
                    ui.add_space(28.0);
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.separator();
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.add_space(28.0);
                        ui.label(egui::RichText::new("●").size(18.0).color(SUCCESS));
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("Pronto para baixar")
                                .size(16.0)
                                .color(MUTED),
                        );
                    });
                    ui.add_space(16.0);
                });
            });
    }
}

fn compact_path(path: &str, max_chars: usize) -> String {
    let char_count = path.chars().count();
    if char_count <= max_chars {
        return path.to_owned();
    }

    let tail_len = max_chars.saturating_sub(1);
    let tail: String = path
        .chars()
        .rev()
        .take(tail_len)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("…{tail}")
}

fn configure_download_theme(ctx: &egui::Context) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct MemoryOutput {
        lines: Vec<String>,
    }

    impl Output for MemoryOutput {
        fn handle_submission(&mut self, pin_url: &str, directory_path: &str) {
            self.lines.push(pin_url.to_owned());
            self.lines.push(directory_path.to_owned());
        }
    }

    #[test]
    fn state_starts_empty() {
        let state = TextInputState::new();

        assert_eq!(state.text(), "");
        assert_eq!(state.directory_path(), "");
    }

    #[test]
    fn state_updates_text() {
        let mut state = TextInputState::new();

        state.set_text("olá mundo");

        assert_eq!(state.text(), "olá mundo");
    }

    #[test]
    fn state_updates_directory_path() {
        let mut state = TextInputState::new();

        state.set_directory_path("/home/pedro/downloads");

        assert_eq!(state.directory_path(), "/home/pedro/downloads");
    }

    #[test]
    fn compact_path_keeps_short_path() {
        assert_eq!(compact_path("/tmp/downloads", 20), "/tmp/downloads");
    }

    #[test]
    fn compact_path_truncates_long_path_from_the_left() {
        assert_eq!(
            compact_path("/home/pedro/downloads/pinterest", 12),
            "…s/pinterest"
        );
    }

    #[test]
    fn submit_sends_current_text_and_directory_path_to_output() {
        let mut app = TextPrinterApp::with_output(MemoryOutput::default());
        app.state_mut().set_text("texto digitado");
        app.state_mut().set_directory_path("/home/pedro/downloads");

        app.submit();

        let output = app.into_output();
        assert_eq!(
            output.lines,
            vec!["texto digitado", "/home/pedro/downloads"]
        );
    }
}

use eframe::egui;

use crate::output::{ConsoleOutput, Output};

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
}

impl<O: Output> eframe::App for TextPrinterApp<O> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        configure_black_theme(ui.ctx());

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::BLACK))
            .show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let text_field = ui.add(
                                    egui::TextEdit::singleline(&mut self.state.text)
                                        .desired_width(320.0)
                                        .hint_text("Digite aqui"),
                                );

                                let enter_pressed = text_field.lost_focus()
                                    && ui.input(|input| input.key_pressed(egui::Key::Enter));
                                let button_clicked = ui.button("Enviar").clicked();

                                if enter_pressed || button_clicked {
                                    self.submit();
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.directory_path)
                                        .desired_width(320.0)
                                        .hint_text("Pasta de destino"),
                                );

                                if ui.button("Escolher pasta").clicked() {
                                    self.choose_directory_path();
                                }
                            });
                        });
                    },
                );
            });
    }
}

fn configure_black_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::BLACK;
    visuals.window_fill = egui::Color32::BLACK;
    visuals.extreme_bg_color = egui::Color32::BLACK;
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

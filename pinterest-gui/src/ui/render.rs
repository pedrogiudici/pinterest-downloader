use eframe::egui;

use pinterest_core::Output;

use crate::icons::{draw_icon, Icon};

use super::app::DownloaderApp;
use super::state::DownloadStatus;
use super::theme::{
    ACCENT, ACCENT_HOVER, BORDER, CARD, CARD_SOFT, DANGER, MUTED, TEXT,
};
use super::widgets::{compact_path, icon_button};

impl<O: Output> DownloaderApp<O> {
    pub(super) fn destination_button_label(&self) -> String {
        match self.state.directory_path() {
            "" => "Selecionar pasta".to_owned(),
            path => compact_path(path, 14),
        }
    }

    pub(super) fn render_download_history(&mut self, ui: &mut egui::Ui) {
        let mut dismissed_download_id = None;
        for download in &self.downloads {
            ui.add_space(14.0);
            egui::Frame::new()
                .fill(CARD)
                .stroke(egui::Stroke::new(1.0, BORDER))
                .corner_radius(egui::CornerRadius::same(12))
                .inner_margin(egui::Margin::symmetric(22, 18))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        draw_icon(ui, Icon::Download, 24.0, ACCENT);
                        ui.add_space(10.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(&download.filename)
                                    .size(17.0)
                                    .strong()
                                    .color(TEXT),
                            );
                            ui.add_space(4.0);
                            let (label, color) = match &download.status {
                                DownloadStatus::Downloading => ("Baixando", ACCENT),
                                DownloadStatus::Completed => {
                                    ("Conclu\u{ed}do", egui::Color32::from_rgb(79, 214, 123))
                                }
                                DownloadStatus::Failed(_) => {
                                    ("Erro", egui::Color32::from_rgb(255, 107, 107))
                                }
                            };
                            ui.label(egui::RichText::new(label).size(14.0).color(color));
                        });

                        if download.status.is_finished() {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if icon_button(
                                        ui,
                                        egui::vec2(128.0, 40.0),
                                        CARD_SOFT,
                                        egui::Stroke::new(1.0, DANGER),
                                        DANGER,
                                        "Remover",
                                        Icon::Trash,
                                    )
                                    .clicked()
                                    {
                                        dismissed_download_id = Some(download.id);
                                    }
                                },
                            );
                        }
                    });
                });
        }

        if let Some(id) = dismissed_download_id {
            self.dismiss_download(id);
        }
    }

    pub(super) fn render_new_download_card(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(egui::CornerRadius::same(12))
            .inner_margin(egui::Margin::symmetric(28, 24))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.horizontal(|ui| {
                    draw_icon(ui, Icon::Download, 30.0, ACCENT);
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Novo Download")
                                .size(22.0)
                                .strong()
                                .color(TEXT),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(
                                "Cole o link do Pinterest que deseja baixar",
                            )
                            .size(15.0)
                            .color(MUTED),
                        );
                    });
                });

                ui.add_space(24.0);

                ui.horizontal(|ui| {
                    let input_width = (ui.available_width() - 371.0).max(240.0);
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(14, 17, 26))
                        .stroke(egui::Stroke::new(1.5, ACCENT))
                        .corner_radius(egui::CornerRadius::same(7))
                        .inner_margin(egui::Margin::symmetric(14, 8))
                        .show(ui, |ui| {
                            ui.set_width(input_width);
                            ui.horizontal(|ui| {
                                draw_icon(ui, Icon::Link, 20.0, MUTED);
                                ui.add_space(8.0);
                                let text_field = ui.add(
                                    egui::TextEdit::singleline(&mut self.state.url)
                                        .desired_width(input_width - 42.0)
                                        .hint_text("https://br.pinterest.com/pin/..."),
                                );

                                let enter_pressed = text_field.lost_focus()
                                    && ui.input(|input| input.key_pressed(egui::Key::Enter));
                                if enter_pressed {
                                    self.submit();
                                }
                            });
                        });

                    ui.add_space(12.0);

                    let folder_label = self.destination_button_label();
                    if icon_button(
                        ui,
                        egui::vec2(180.0, 48.0),
                        CARD_SOFT,
                        egui::Stroke::new(1.0, BORDER),
                        TEXT,
                        &folder_label,
                        Icon::Folder,
                    )
                    .clicked()
                    {
                        self.choose_directory_path();
                    }

                    ui.add_space(12.0);

                    if icon_button(
                        ui,
                        egui::vec2(136.0, 48.0),
                        ACCENT,
                        egui::Stroke::new(1.0, ACCENT_HOVER),
                        TEXT,
                        "Baixar",
                        Icon::Download,
                    )
                    .clicked()
                    {
                        self.submit();
                    }
                });
            });
    }
}

use std::time::Duration;

use eframe::egui;

use crate::output::{ConsoleOutput, DownloadEvent, DownloadEventKind, DownloadId, Output};

const BACKGROUND: egui::Color32 = egui::Color32::from_rgb(13, 16, 24);
const CARD: egui::Color32 = egui::Color32::from_rgb(19, 23, 33);
const CARD_SOFT: egui::Color32 = egui::Color32::from_rgb(24, 29, 41);
const BORDER: egui::Color32 = egui::Color32::from_rgb(37, 43, 58);
const TEXT: egui::Color32 = egui::Color32::from_rgb(238, 241, 247);
const MUTED: egui::Color32 = egui::Color32::from_rgb(165, 171, 187);
const ACCENT: egui::Color32 = egui::Color32::from_rgb(105, 76, 255);
const ACCENT_HOVER: egui::Color32 = egui::Color32::from_rgb(125, 91, 255);

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Downloading,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadCardState {
    id: DownloadId,
    filename: String,
    status: DownloadStatus,
}

impl DownloadCardState {
    pub fn id(&self) -> DownloadId {
        self.id
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn status(&self) -> &DownloadStatus {
        &self.status
    }
}

#[derive(Debug)]
pub struct TextPrinterApp<O = ConsoleOutput> {
    state: TextInputState,
    output: O,
    downloads: Vec<DownloadCardState>,
    next_download_id: DownloadId,
}

impl TextPrinterApp<ConsoleOutput> {
    pub fn new() -> Self {
        Self::with_output(ConsoleOutput::default())
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
            downloads: Vec::new(),
            next_download_id: 1,
        }
    }

    pub fn state(&self) -> &TextInputState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut TextInputState {
        &mut self.state
    }

    pub fn submit(&mut self) {
        let id = self.next_download_id;
        self.next_download_id += 1;
        self.downloads.push(DownloadCardState {
            id,
            filename: "Preparando download...".to_owned(),
            status: DownloadStatus::Downloading,
        });
        self.output
            .handle_submission(self.state.text(), self.state.directory_path(), id);
    }

    pub fn downloads(&self) -> &[DownloadCardState] {
        &self.downloads
    }

    pub fn apply_download_events(&mut self) {
        let events = self.output.drain_events();
        self.apply_events(events);
    }

    fn apply_events(&mut self, events: Vec<DownloadEvent>) {
        for event in events {
            let Some(download) = self
                .downloads
                .iter_mut()
                .find(|download| download.id == event.id)
            else {
                continue;
            };

            match event.kind {
                DownloadEventKind::FileNameResolved(filename) => download.filename = filename,
                DownloadEventKind::Completed => download.status = DownloadStatus::Completed,
                DownloadEventKind::Failed(error) => download.status = DownloadStatus::Failed(error),
            }
        }
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
            "" => "Selecionar pasta".to_owned(),
            path => compact_path(path, 14),
        }
    }

    fn render_download_list(&self, ui: &mut egui::Ui) {
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
                        draw_download_icon(ui, 24.0, ACCENT);
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
                                    ("Concluído", egui::Color32::from_rgb(79, 214, 123))
                                }
                                DownloadStatus::Failed(_) => {
                                    ("Erro", egui::Color32::from_rgb(255, 107, 107))
                                }
                            };
                            ui.label(egui::RichText::new(label).size(14.0).color(color));
                        });
                    });
                });
        }
    }

    fn render_new_download_card(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(egui::CornerRadius::same(12))
            .inner_margin(egui::Margin::symmetric(28, 24))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.horizontal(|ui| {
                    draw_download_icon(ui, 30.0, ACCENT);
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
                            egui::RichText::new("Cole o link do Pinterest que deseja baixar")
                                .size(15.0)
                                .color(MUTED),
                        );
                    });
                });

                ui.add_space(24.0);

                ui.horizontal(|ui| {
                    let input_width = (ui.available_width() - 360.0).max(240.0);
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(14, 17, 26))
                        .stroke(egui::Stroke::new(1.5, ACCENT))
                        .corner_radius(egui::CornerRadius::same(7))
                        .inner_margin(egui::Margin::symmetric(14, 8))
                        .show(ui, |ui| {
                            ui.set_width(input_width);
                            ui.horizontal(|ui| {
                                draw_link_icon(ui, 20.0, MUTED);
                                ui.add_space(8.0);
                                let text_field = ui.add(
                                    egui::TextEdit::singleline(&mut self.state.text)
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
                        paint_folder_icon,
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
                        egui::Color32::WHITE,
                        "Baixar",
                        paint_download_icon,
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
        self.apply_download_events();
        if self
            .downloads
            .iter()
            .any(|download| matches!(download.status, DownloadStatus::Downloading))
        {
            ui.ctx().request_repaint_after(Duration::from_millis(250));
        }
        configure_download_theme(ui.ctx());

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BACKGROUND))
            .show(ui, |ui| {
                let available = ui.available_rect_before_wrap();
                let content_rect = egui::Rect::from_min_max(
                    egui::pos2(available.left() + 22.0, available.top() + 22.0),
                    egui::pos2(available.right() - 28.0, available.bottom()),
                );

                ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                    ui.set_width(content_rect.width());
                    self.render_new_download_card(ui);

                    let scroll_rect = ui.available_rect_before_wrap();
                    ui.scope_builder(egui::UiBuilder::new().max_rect(scroll_rect), |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.set_width(scroll_rect.width());
                                self.render_download_list(ui);
                            });
                    });
                });
            });
    }
}

fn icon_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    fill: egui::Color32,
    stroke: egui::Stroke,
    text_color: egui::Color32,
    label: &str,
    paint_icon: fn(&egui::Painter, egui::Rect, egui::Color32),
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let fill = if response.hovered() {
        fill.gamma_multiply(1.15)
    } else {
        fill
    };

    ui.painter().rect(
        rect,
        egui::CornerRadius::same(8),
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 30.0, rect.center().y),
        egui::vec2(20.0, 20.0),
    );
    paint_icon(ui.painter(), icon_rect, text_color);
    ui.painter().with_clip_rect(rect.shrink(8.0)).text(
        egui::pos2(rect.left() + 54.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(14.0),
        text_color,
    );

    response
}

fn draw_download_icon(ui: &mut egui::Ui, size: f32, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    paint_download_icon(ui.painter(), rect, color);
}

fn paint_download_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let size = rect.width().min(rect.height());
    let stroke = egui::Stroke::new((size / 12.0).max(1.5), color);
    let center_x = rect.center().x;
    let top = rect.top() + size * 0.18;
    let mid = rect.top() + size * 0.58;
    painter.line_segment(
        [egui::pos2(center_x, top), egui::pos2(center_x, mid)],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(center_x - size * 0.18, mid - size * 0.18),
            egui::pos2(center_x, mid),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(center_x + size * 0.18, mid - size * 0.18),
            egui::pos2(center_x, mid),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(rect.left() + size * 0.24, rect.bottom() - size * 0.18),
            egui::pos2(rect.right() - size * 0.24, rect.bottom() - size * 0.18),
        ],
        stroke,
    );
}

fn draw_link_icon(ui: &mut egui::Ui, size: f32, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    let painter = ui.painter();
    let stroke = egui::Stroke::new((size / 12.0).max(1.5), color);
    painter.line_segment(
        [
            egui::pos2(rect.left() + size * 0.33, rect.bottom() - size * 0.33),
            egui::pos2(rect.right() - size * 0.33, rect.top() + size * 0.33),
        ],
        stroke,
    );
    painter.circle_stroke(
        egui::pos2(rect.left() + size * 0.34, rect.bottom() - size * 0.34),
        size * 0.18,
        stroke,
    );
    painter.circle_stroke(
        egui::pos2(rect.right() - size * 0.34, rect.top() + size * 0.34),
        size * 0.18,
        stroke,
    );
}

fn paint_folder_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let size = rect.width().min(rect.height());
    let stroke = egui::Stroke::new((size / 13.0).max(1.4), color);
    let points = vec![
        egui::pos2(rect.left() + size * 0.14, rect.top() + size * 0.34),
        egui::pos2(rect.left() + size * 0.40, rect.top() + size * 0.34),
        egui::pos2(rect.left() + size * 0.48, rect.top() + size * 0.44),
        egui::pos2(rect.right() - size * 0.14, rect.top() + size * 0.44),
        egui::pos2(rect.right() - size * 0.14, rect.bottom() - size * 0.18),
        egui::pos2(rect.left() + size * 0.14, rect.bottom() - size * 0.18),
        egui::pos2(rect.left() + size * 0.14, rect.top() + size * 0.34),
    ];
    painter.add(egui::Shape::line(points, stroke));
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
        ids: Vec<DownloadId>,
        events: Vec<DownloadEvent>,
    }

    impl Output for MemoryOutput {
        fn handle_submission(&mut self, pin_url: &str, directory_path: &str, id: DownloadId) {
            self.lines.push(pin_url.to_owned());
            self.lines.push(directory_path.to_owned());
            self.ids.push(id);
        }

        fn drain_events(&mut self) -> Vec<DownloadEvent> {
            self.events.drain(..).collect()
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
    fn destination_button_label_keeps_long_path_inside_button() {
        let mut app = TextPrinterApp::with_output(MemoryOutput::default());
        app.state_mut()
            .set_directory_path("/home/pedro/downloads/pinterest/videos");

        assert_eq!(app.destination_button_label(), "…terest/videos");
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
        assert_eq!(output.ids, vec![1]);
    }

    #[test]
    fn submit_adds_download_card_as_downloading() {
        let mut app = TextPrinterApp::with_output(MemoryOutput::default());

        app.submit();

        assert_eq!(app.downloads().len(), 1);
        assert_eq!(app.downloads()[0].id(), 1);
        assert_eq!(app.downloads()[0].filename(), "Preparando download...");
        assert_eq!(app.downloads()[0].status(), &DownloadStatus::Downloading);
    }

    #[test]
    fn download_events_update_card_filename_and_status() {
        let mut app = TextPrinterApp::with_output(MemoryOutput {
            events: vec![
                DownloadEvent {
                    id: 1,
                    kind: DownloadEventKind::FileNameResolved("video.mp4".to_owned()),
                },
                DownloadEvent {
                    id: 1,
                    kind: DownloadEventKind::Completed,
                },
            ],
            ..Default::default()
        });
        app.submit();

        app.apply_download_events();

        assert_eq!(app.downloads()[0].filename(), "video.mp4");
        assert_eq!(app.downloads()[0].status(), &DownloadStatus::Completed);
    }
}

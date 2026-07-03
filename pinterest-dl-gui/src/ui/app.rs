use std::time::Duration;

use eframe::egui;

use pinterest_dl_core::{DownloadEvent, DownloadEventKind, DownloadId, DownloadWorker, Output};

use super::state::{DownloadCardState, DownloadStatus, TextInputState};
use super::theme::{configure_download_theme, BACKGROUND};

#[derive(Debug)]
pub struct DownloaderApp<O = DownloadWorker> {
    pub(super) state: TextInputState,
    pub(super) output: O,
    pub(super) downloads: Vec<DownloadCardState>,
    pub(super) next_download_id: DownloadId,
}

impl DownloaderApp<DownloadWorker> {
    pub fn new() -> Self {
        Self::with_output(DownloadWorker::default())
    }
}

impl Default for DownloaderApp<DownloadWorker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<O: Output> DownloaderApp<O> {
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
            .handle_submission(self.state.url(), self.state.directory_path(), id);
    }

    pub fn downloads(&self) -> &[DownloadCardState] {
        &self.downloads
    }

    pub fn dismiss_download(&mut self, id: DownloadId) {
        self.downloads
            .retain(|download| download.id != id || !download.status.is_finished());
    }

    pub fn clear_finished_downloads(&mut self) {
        self.downloads
            .retain(|download| !download.status.is_finished());
    }

    pub fn apply_download_events(&mut self) {
        let events = self.output.drain_events();
        self.apply_events(events);
    }

    pub(super) fn apply_events(&mut self, events: Vec<DownloadEvent>) {
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
}

impl<O: Output> eframe::App for DownloaderApp<O> {
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
                                self.render_download_history(ui);
                            });
                    });
                });
            });
    }
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
    fn destination_button_label_keeps_long_path_inside_button() {
        let mut app = DownloaderApp::with_output(MemoryOutput::default());
        app.state_mut()
            .set_directory_path("/home/pedro/downloads/pinterest/videos");

        assert_eq!(app.destination_button_label(), "\u{2026}terest/videos");
    }

    #[test]
    fn submit_sends_current_text_and_directory_path_to_output() {
        let mut app = DownloaderApp::with_output(MemoryOutput::default());
        app.state_mut().set_url("https://br.pinterest.com/pin/123");
        app.state_mut().set_directory_path("/home/pedro/downloads");

        app.submit();

        let output = app.into_output();
        assert_eq!(
            output.lines,
            vec!["https://br.pinterest.com/pin/123", "/home/pedro/downloads"]
        );
        assert_eq!(output.ids, vec![1]);
    }

    #[test]
    fn submit_adds_download_card_as_downloading() {
        let mut app = DownloaderApp::with_output(MemoryOutput::default());

        app.submit();

        assert_eq!(app.downloads().len(), 1);
        assert_eq!(app.downloads()[0].id(), 1);
        assert_eq!(app.downloads()[0].filename(), "Preparando download...");
        assert_eq!(app.downloads()[0].status(), &DownloadStatus::Downloading);
    }

    #[test]
    fn download_events_update_card_filename_and_status() {
        let mut app = DownloaderApp::with_output(MemoryOutput {
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

    #[test]
    fn dismiss_download_removes_finished_download_from_history() {
        let mut app = DownloaderApp::with_output(MemoryOutput::default());
        app.submit();
        app.apply_events(vec![DownloadEvent {
            id: 1,
            kind: DownloadEventKind::Completed,
        }]);

        app.dismiss_download(1);

        assert!(app.downloads().is_empty());
    }

    #[test]
    fn dismiss_download_keeps_active_download() {
        let mut app = DownloaderApp::with_output(MemoryOutput::default());
        app.submit();

        app.dismiss_download(1);

        assert_eq!(app.downloads().len(), 1);
        assert_eq!(app.downloads()[0].status(), &DownloadStatus::Downloading);
    }

    #[test]
    fn clear_finished_downloads_keeps_active_downloads() {
        let mut app = DownloaderApp::with_output(MemoryOutput::default());
        app.submit();
        app.submit();
        app.apply_events(vec![DownloadEvent {
            id: 1,
            kind: DownloadEventKind::Completed,
        }]);

        app.clear_finished_downloads();

        assert_eq!(app.downloads().len(), 1);
        assert_eq!(app.downloads()[0].id(), 2);
        assert_eq!(app.downloads()[0].status(), &DownloadStatus::Downloading);
    }
}

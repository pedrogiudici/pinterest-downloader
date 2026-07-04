use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::downloader::{download_video, extract_download_url, filename_from_url};

pub type DownloadId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadEvent {
    pub id: DownloadId,
    pub kind: DownloadEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadEventKind {
    FileNameResolved(String),
    Completed,
    Failed(String),
}

pub trait Output {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str, id: DownloadId);

    fn drain_events(&mut self) -> Vec<DownloadEvent> {
        Vec::new()
    }
}

#[derive(Debug, Default, Clone)]
pub struct DownloadWorker {
    events: Arc<Mutex<Vec<DownloadEvent>>>,
}

impl Output for DownloadWorker {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str, id: DownloadId) {
        let pin_url = pin_url.to_owned();
        let directory_path = directory_path.to_owned();
        let events = Arc::clone(&self.events);

        std::thread::spawn(move || {
            let emit = |kind: DownloadEventKind| {
                if let Ok(mut events) = events.lock() {
                    events.push(DownloadEvent { id, kind });
                }
            };

            match extract_download_url(&pin_url) {
                Ok(download_url) => {
                    let filename =
                        filename_from_url(&download_url).unwrap_or_else(|| "video.mp4".to_owned());
                    emit(DownloadEventKind::FileNameResolved(filename.clone()));

                    let dest = PathBuf::from(&directory_path).join(&filename);

                    match download_video(&download_url, &dest) {
                        Ok(()) => emit(DownloadEventKind::Completed),
                        Err(e) => emit(DownloadEventKind::Failed(format!("{e:?}"))),
                    }
                }
                Err(e) => emit(DownloadEventKind::Failed(format!("{e:?}"))),
            }
        });
    }

    fn drain_events(&mut self) -> Vec<DownloadEvent> {
        let Ok(mut events) = self.events.lock() else {
            return Vec::new();
        };
        events.drain(..).collect()
    }
}

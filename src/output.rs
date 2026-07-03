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

/// Abstrai o processamento do formulário enviado pela interface.
///
/// Em produção, a implementação padrão extrai o link de download a partir da
/// URL do pin e baixa o vídeo para o diretório escolhido pelo usuário.
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

impl DownloadWorker {
    fn push_event(events: &Arc<Mutex<Vec<DownloadEvent>>>, event: DownloadEvent) {
        if let Ok(mut events) = events.lock() {
            events.push(event);
        }
    }
}

impl Output for DownloadWorker {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str, id: DownloadId) {
        let pin_url = pin_url.to_owned();
        let directory_path = directory_path.to_owned();
        let events = Arc::clone(&self.events);

        std::thread::spawn(move || match extract_download_url(&pin_url) {
            Ok(download_url) => {
                let filename =
                    filename_from_url(&download_url).unwrap_or_else(|| "video.mp4".to_owned());
                Self::push_event(
                    &events,
                    DownloadEvent {
                        id,
                        kind: DownloadEventKind::FileNameResolved(filename.clone()),
                    },
                );

                let dest = PathBuf::from(&directory_path).join(&filename);

                match download_video(&download_url, &dest) {
                    Ok(()) => Self::push_event(
                        &events,
                        DownloadEvent {
                            id,
                            kind: DownloadEventKind::Completed,
                        },
                    ),
                    Err(e) => Self::push_event(
                        &events,
                        DownloadEvent {
                            id,
                            kind: DownloadEventKind::Failed(format!("{e:?}")),
                        },
                    ),
                }
            }
            Err(e) => Self::push_event(
                &events,
                DownloadEvent {
                    id,
                    kind: DownloadEventKind::Failed(format!("{e:?}")),
                },
            ),
        });
    }

    fn drain_events(&mut self) -> Vec<DownloadEvent> {
        let Ok(mut events) = self.events.lock() else {
            return Vec::new();
        };
        events.drain(..).collect()
    }
}

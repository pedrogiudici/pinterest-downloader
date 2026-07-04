use pinterest_dl_core::DownloadId;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextInputState {
    pub(super) url: String,
    pub(super) directory_path: String,
}

impl TextInputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(&self) -> &str {
        &self.url
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

impl DownloadStatus {
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadCardState {
    pub(super) id: DownloadId,
    pub(super) filename: String,
    pub(super) status: DownloadStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_starts_empty() {
        let state = TextInputState::new();

        assert_eq!(state.url(), "");
        assert_eq!(state.directory_path(), "");
    }

    #[test]
    fn state_updates_url() {
        let mut state = TextInputState::new();

        state.url = "https://br.pinterest.com/pin/123".to_owned();

        assert_eq!(state.url(), "https://br.pinterest.com/pin/123");
    }

    #[test]
    fn state_updates_directory_path() {
        let mut state = TextInputState::new();

        state.set_directory_path("/home/pedro/downloads");

        assert_eq!(state.directory_path(), "/home/pedro/downloads");
    }
}

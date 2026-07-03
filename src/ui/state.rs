use crate::output::DownloadId;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextInputState {
    pub(super) text: String,
    pub(super) directory_path: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_starts_empty() {
        let state = TextInputState::new();

        assert_eq!(state.text(), "");
        assert_eq!(state.directory_path(), "");
    }

    #[test]
    fn state_updates_text() {
        let mut state = TextInputState::new();

        state.set_text("ol\u{e1} mundo");

        assert_eq!(state.text(), "ol\u{e1} mundo");
    }

    #[test]
    fn state_updates_directory_path() {
        let mut state = TextInputState::new();

        state.set_directory_path("/home/pedro/downloads");

        assert_eq!(state.directory_path(), "/home/pedro/downloads");
    }
}

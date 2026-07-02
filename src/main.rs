use pinterest_downloader::TextPrinterApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Downloader",
        options,
        Box::new(|_creation_context| Ok(Box::new(TextPrinterApp::new()))),
    )
}

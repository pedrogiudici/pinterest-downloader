use pinterest_downloader::TextPrinterApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Downloader",
        options,
        Box::new(|creation_context| {
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Ok(Box::new(TextPrinterApp::new()))
        }),
    )
}

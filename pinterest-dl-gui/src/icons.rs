use eframe::egui;

#[derive(Clone, Copy)]
pub enum Icon {
    Download,
    Folder,
    Link,
    Trash,
}

fn icon_svg_str(icon: Icon) -> &'static str {
    match icon {
        Icon::Download => include_str!("../assets/icons/arrow-down-to-line.svg"),
        Icon::Folder => include_str!("../assets/icons/folder.svg"),
        Icon::Link => include_str!("../assets/icons/link.svg"),
        Icon::Trash => include_str!("../assets/icons/trash-2.svg"),
    }
}

pub fn colored_source(icon: Icon, color: egui::Color32) -> egui::ImageSource<'static> {
    let svg = icon_svg_str(icon);
    let hex = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
    let colored = svg.replace("currentColor", &hex);
    let icon_name = match icon {
        Icon::Download => "download",
        Icon::Folder => "folder",
        Icon::Link => "link",
        Icon::Trash => "trash",
    };
    let uri = format!("bytes://icon/{icon_name}/{}.svg", &hex[1..]);
    egui::ImageSource::Bytes {
        uri: std::borrow::Cow::Owned(uri),
        bytes: egui::load::Bytes::from(colored.into_bytes()),
    }
}

pub fn draw_icon(ui: &mut egui::Ui, icon: Icon, size: f32, color: egui::Color32) {
    ui.add_sized(
        egui::vec2(size, size),
        egui::Image::new(colored_source(icon, color)),
    );
}

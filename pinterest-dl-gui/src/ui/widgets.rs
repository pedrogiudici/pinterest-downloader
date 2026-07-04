use eframe::egui;

use crate::icons::{Icon, colored_source};

pub fn icon_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    fill: egui::Color32,
    stroke: egui::Stroke,
    text_color: egui::Color32,
    label: &str,
    icon: Icon,
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
    egui::Image::new(colored_source(icon, text_color)).paint_at(ui, icon_rect);
    ui.painter().with_clip_rect(rect.shrink(8.0)).text(
        egui::pos2(rect.left() + 54.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(14.0),
        text_color,
    );

    response
}

pub fn compact_path(path: &str, max_chars: usize) -> String {
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
    format!("\u{2026}{tail}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_path_keeps_short_path() {
        assert_eq!(compact_path("/tmp/downloads", 20), "/tmp/downloads");
    }

    #[test]
    fn compact_path_truncates_long_path_from_the_left() {
        assert_eq!(
            compact_path("/home/pedro/downloads/pinterest", 12),
            "\u{2026}s/pinterest"
        );
    }
}

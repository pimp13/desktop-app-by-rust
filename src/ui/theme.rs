use egui::{Color32, CornerRadius, FontFamily, FontId, Stroke, Style, TextStyle, Visuals};

#[derive(Clone, Copy)]
pub struct Colors;

impl Colors {
    pub const BG: Color32 = Color32::from_rgb(12, 16, 22);
    pub const PANEL: Color32 = Color32::from_rgb(22, 28, 38);
    pub const PANEL_ELEVATED: Color32 = Color32::from_rgb(28, 36, 48);
    pub const BORDER: Color32 = Color32::from_rgb(42, 52, 68);
    pub const ACCENT: Color32 = Color32::from_rgb(45, 168, 148);
    pub const ACCENT_MUTED: Color32 = Color32::from_rgb(28, 72, 66);
    pub const TEXT: Color32 = Color32::from_rgb(232, 238, 244);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(132, 148, 168);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(90, 104, 122);
    pub const ERROR: Color32 = Color32::from_rgb(224, 90, 90);
    pub const ERROR_BG: Color32 = Color32::from_rgb(56, 28, 32);
    pub const SUCCESS: Color32 = Color32::from_rgb(82, 186, 122);
    pub const WARNING: Color32 = Color32::from_rgb(220, 168, 72);
    pub const INPUT_BG: Color32 = Color32::from_rgb(16, 20, 28);
    pub const SIDEBAR: Color32 = Color32::from_rgb(16, 22, 30);
}

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = Style::default();

    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(28.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(15.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(15.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(12.5, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(13.0, FontFamily::Monospace),
        ),
    ]
    .into();

    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(16.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(0);
    style.visuals = dark_visuals();

    ctx.set_style(style);
}

fn dark_visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.dark_mode = true;
    v.override_text_color = Some(Colors::TEXT);
    v.panel_fill = Colors::BG;
    v.window_fill = Colors::PANEL;
    v.extreme_bg_color = Colors::INPUT_BG;
    v.faint_bg_color = Colors::PANEL_ELEVATED;
    v.widgets.noninteractive.bg_fill = Colors::PANEL;
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, Colors::TEXT_MUTED);
    v.widgets.inactive.bg_fill = Colors::PANEL_ELEVATED;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, Colors::TEXT);
    v.widgets.inactive.weak_bg_fill = Colors::PANEL_ELEVATED;
    v.widgets.hovered.bg_fill = Colors::ACCENT_MUTED;
    v.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Colors::TEXT);
    v.widgets.active.bg_fill = Colors::ACCENT;
    v.widgets.active.fg_stroke = Stroke::new(1.0_f32, Colors::BG);
    v.selection.bg_fill = Colors::ACCENT.gamma_multiply(0.35);
    v.selection.stroke = Stroke::new(1.0_f32, Colors::ACCENT);
    v.window_corner_radius = CornerRadius::same(16);
    v.menu_corner_radius = CornerRadius::same(10);
    v.widgets.noninteractive.corner_radius = CornerRadius::same(10);
    v.widgets.inactive.corner_radius = CornerRadius::same(10);
    v.widgets.hovered.corner_radius = CornerRadius::same(10);
    v.widgets.active.corner_radius = CornerRadius::same(10);
    v.window_stroke = Stroke::new(1.0_f32, Colors::BORDER);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Colors::BORDER);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, Colors::BORDER);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, Colors::ACCENT);
    v
}

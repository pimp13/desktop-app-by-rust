use egui::{
    Align, Color32, CornerRadius, Frame, Layout, Margin, Response, RichText, Sense, Stroke, Ui,
    Vec2,
};

use super::rtl::{fa, fa_rich, rtl_column, rtl_row};
use super::theme::Colors;
use crate::auth::PasswordStrength;

pub fn brand_mark(ui: &mut Ui, large: bool) {
    let size = if large { 42.0 } else { 28.0 };
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same((size * 0.28) as u8), Colors::ACCENT);
    let inner = rect.shrink(size * 0.22);
    painter.circle_filled(inner.center(), inner.width() * 0.22, Colors::BG);
    painter.circle_stroke(
        inner.center(),
        inner.width() * 0.38,
        Stroke::new(2.0_f32, Colors::BG),
    );
}

pub fn labeled_text(
    ui: &mut Ui,
    id: &str,
    label: &str,
    hint: &str,
    value: &mut String,
    password: bool,
) -> Response {
    ui.add_space(2.0);
    ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
        ui.label(fa_rich(label).size(12.5).color(Colors::TEXT_MUTED));
    });
    ui.add_space(2.0);

    let mut edit = egui::TextEdit::singleline(value)
        .id_source(id)
        .hint_text(fa_rich(hint).color(Colors::TEXT_DIM))
        .desired_width(f32::INFINITY)
        .margin(Margin::symmetric(12, 8))
        .horizontal_align(Align::Max)
        .frame(false);

    if password {
        edit = edit.password(true);
    }

    Frame::new()
        .fill(Colors::INPUT_BG)
        .stroke(Stroke::new(1.0_f32, Colors::BORDER))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| ui.add(edit))
        .inner
}

pub fn primary_button(ui: &mut Ui, label: &str, enabled: bool) -> Response {
    let text = fa_rich(label).size(15.0).strong().color(if enabled {
        Colors::BG
    } else {
        Colors::TEXT_DIM
    });

    let fill = if enabled {
        Colors::ACCENT
    } else {
        Colors::PANEL_ELEVATED
    };

    let mut button = egui::Button::new(text)
        .fill(fill)
        .corner_radius(CornerRadius::same(8))
        .min_size(Vec2::new(ui.available_width(), 38.0));

    if enabled {
        button = button.stroke(Stroke::NONE);
    } else {
        button = button.stroke(Stroke::new(1.0_f32, Colors::BORDER));
    }

    let response = ui.add_enabled(enabled, button);
    if response.hovered() && enabled {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response
}

pub fn ghost_button(ui: &mut Ui, label: &str) -> Response {
    let response = ui.add(
        egui::Button::new(fa_rich(label).size(14.0).color(Colors::ACCENT))
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::NONE),
    );
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response
}

pub fn error_banner(ui: &mut Ui, message: &str) {
    if message.is_empty() {
        return;
    }
    Frame::new()
        .fill(Colors::ERROR_BG)
        .stroke(Stroke::new(1.0_f32, Colors::ERROR.gamma_multiply(0.5)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
                ui.label(fa_rich(message).color(Colors::ERROR).size(12.5));
            });
        });
    ui.add_space(4.0);
}

pub fn success_banner(ui: &mut Ui, message: &str) {
    if message.is_empty() {
        return;
    }
    Frame::new()
        .fill(Color32::from_rgb(22, 42, 32))
        .stroke(Stroke::new(1.0_f32, Colors::SUCCESS.gamma_multiply(0.45)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
                ui.label(fa_rich(message).color(Colors::SUCCESS).size(12.5));
            });
        });
    ui.add_space(4.0);
}

pub fn strength_bar(ui: &mut Ui, strength: PasswordStrength) {
    let (label, color) = match strength {
        PasswordStrength::Weak => (strength.label_fa(), Colors::ERROR),
        PasswordStrength::Fair => (strength.label_fa(), Colors::WARNING),
        PasswordStrength::Good => (strength.label_fa(), Colors::ACCENT),
        PasswordStrength::Strong => (strength.label_fa(), Colors::SUCCESS),
    };

    rtl_row(ui, |ui| {
        ui.label(fa_rich(label).size(12.5).color(color).strong());
        ui.label(fa_rich("قدرت رمز:").size(12.5).color(Colors::TEXT_MUTED));
    });

    let height = 5.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(3), Colors::BORDER);
    // Fill from the right in RTL UIs
    let filled_w = rect.width() * strength.score();
    let filled = egui::Rect::from_min_max(
        egui::pos2(rect.max.x - filled_w, rect.min.y),
        rect.max,
    );
    painter.rect_filled(filled, CornerRadius::same(3), color);
}

pub fn auth_shell(ui: &mut Ui, form_width: f32, add_contents: impl FnOnce(&mut Ui)) {
    let available = ui.available_size();
    let side_w = (available.x * 0.38).clamp(220.0, 360.0);

    ui.allocate_ui_with_layout(available, Layout::right_to_left(Align::Center), |ui| {
        // Form panel (right side in RTL)
        ui.allocate_ui_with_layout(
            Vec2::new(available.x - side_w, available.y),
            Layout::top_down(Align::Center),
            |ui| {
                Frame::new()
                    .fill(Colors::BG)
                    .inner_margin(Margin::symmetric(16, 12))
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(
                            (available.x - side_w).max(0.0),
                            available.y,
                        ));
                        let max_h = (available.y - 24.0).max(120.0);
                        egui::ScrollArea::vertical()
                            .max_height(max_h)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.set_max_width(form_width.min(ui.available_width()));
                                    add_contents(ui);
                                });
                            });
                    });
            },
        );

        // Brand panel
        ui.allocate_ui_with_layout(
            Vec2::new(side_w, available.y),
            Layout::top_down(Align::Center),
            |ui| {
                Frame::new().fill(Colors::SIDEBAR).show(ui, |ui| {
                    ui.set_min_size(Vec2::new(side_w, available.y));
                    ui.vertical_centered(|ui| {
                        ui.add_space((available.y * 0.16).min(80.0));
                        brand_mark(ui, true);
                        ui.add_space(14.0);
                        ui.label(
                            RichText::new("SecureDesk")
                                .size(26.0)
                                .strong()
                                .color(Colors::TEXT),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            fa_rich("احراز هویت مدرن برای دسکتاپ")
                                .size(13.0)
                                .color(Colors::TEXT_MUTED),
                        );
                        ui.add_space(20.0);

                        rtl_column(ui, |ui| {
                            for line in [
                                "هش رمز با Argon2",
                                "قفل حساب پس از تلاش‌های ناموفق",
                                "نشست امن با انقضای زمانی",
                                "پایگاه‌داده محلی SQLite",
                            ] {
                                ui.add_space(4.0);
                                ui.label(
                                    fa_rich(format!("• {line}"))
                                        .size(12.5)
                                        .color(Colors::TEXT_DIM),
                                );
                            }
                        });
                    });
                });
            },
        );
    });
}

pub fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::new()
        .fill(Colors::PANEL)
        .stroke(Stroke::new(1.0_f32, Colors::BORDER))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(Margin::symmetric(20, 16))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 6.0;
            ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
                add_contents(ui);
            });
        });
}

#[allow(dead_code)]
pub fn fa_label(ui: &mut Ui, text: &str) {
    ui.label(fa(text));
}

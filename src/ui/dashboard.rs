use egui::{Align, CornerRadius, Frame, Layout, Margin, RichText, Sense, Stroke, Ui, Vec2};

use super::rtl::{fa, fa_rich, rtl_row};
use super::theme::Colors;
use super::widgets::{brand_mark, primary_button};
use crate::app::AuthApp;

pub fn show_dashboard(app: &mut AuthApp, ui: &mut Ui) {
    let Some(session) = app.session.clone() else {
        return;
    };

    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        // Sidebar on the right (RTL)
        Frame::new()
            .fill(Colors::SIDEBAR)
            .stroke(Stroke::new(1.0_f32, Colors::BORDER))
            .inner_margin(Margin::symmetric(20, 22))
            .show(ui, |ui| {
                ui.set_min_width(220.0);
                ui.set_max_width(220.0);
                ui.set_min_height(ui.available_height());

                rtl_row(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.with_layout(Layout::top_down(Align::Max), |ui| {
                            ui.label(
                                RichText::new("SecureDesk")
                                    .size(16.0)
                                    .strong()
                                    .color(Colors::TEXT),
                            );
                            ui.label(
                                fa_rich("احراز شده")
                                    .size(11.5)
                                    .color(Colors::ACCENT),
                            );
                        });
                    });
                    ui.add_space(8.0);
                    brand_mark(ui, false);
                });

                ui.add_space(28.0);
                nav_item(ui, "داشبورد", true);
                nav_item(ui, "امنیت", false);
                nav_item(ui, "تنظیمات", false);

                ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                    ui.add_space(12.0);
                    if ui
                        .add(
                            egui::Button::new(
                                fa_rich("خروج از حساب").size(13.5).color(Colors::ERROR),
                            )
                            .fill(Colors::ERROR_BG)
                            .stroke(Stroke::new(1.0_f32, Colors::ERROR.gamma_multiply(0.4)))
                            .corner_radius(CornerRadius::same(8))
                            .min_size(Vec2::new(ui.available_width(), 38.0)),
                        )
                        .clicked()
                    {
                        app.logout();
                    }
                });
            });

        // Main content
        Frame::new()
            .fill(Colors::BG)
            .inner_margin(Margin::symmetric(36, 28))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.set_min_height(ui.available_height());

                ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
                    ui.label(
                        fa_rich(format!("سلام، {}", session.display_name))
                            .size(28.0)
                            .strong()
                            .color(Colors::TEXT),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        fa_rich("ورود شما با موفقیت تأیید شد. نشست فعلی امن و فعال است.")
                            .size(14.0)
                            .color(Colors::TEXT_MUTED),
                    );

                    ui.add_space(28.0);

                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        info_card(ui, "نام کاربری", &session.username, Colors::ACCENT);
                        info_card(ui, "ایمیل", &session.email, Colors::WARNING);
                        info_card(
                            ui,
                            "انقضای نشست",
                            &session.expires_at.format("%Y-%m-%d %H:%M UTC").to_string(),
                            Colors::SUCCESS,
                        );
                    });

                    ui.add_space(24.0);

                    Frame::new()
                        .fill(Colors::PANEL)
                        .stroke(Stroke::new(1.0_f32, Colors::BORDER))
                        .corner_radius(CornerRadius::same(14))
                        .inner_margin(Margin::symmetric(22, 20))
                        .show(ui, |ui| {
                            ui.with_layout(
                                Layout::top_down(Align::Max).with_cross_justify(true),
                                |ui| {
                                    ui.label(
                                        fa_rich("وضعیت امنیتی")
                                            .size(16.0)
                                            .strong()
                                            .color(Colors::TEXT),
                                    );
                                    ui.add_space(12.0);
                                    status_row(ui, "هش رمز عبور", "Argon2id", true);
                                    status_row(ui, "قفل ضد Brute-force", "فعال", true);
                                    status_row(
                                        ui,
                                        "نشست پایدار",
                                        if app.remember_me {
                                            "فعال"
                                        } else {
                                            "غیرفعال این ورود"
                                        },
                                        true,
                                    );
                                    status_row(
                                        ui,
                                        "توکن نشست",
                                        &truncate(&session.token, 18),
                                        true,
                                    );
                                },
                            );
                        });

                    ui.add_space(20.0);
                    if primary_button(ui, "تازه‌سازی اطلاعات کاربر", !app.busy).clicked() {
                        app.refresh_profile();
                    }

                    if !app.info_message.is_empty() {
                        ui.add_space(12.0);
                        ui.label(fa_rich(&app.info_message).color(Colors::SUCCESS));
                    }
                    if !app.error_message.is_empty() {
                        ui.add_space(8.0);
                        ui.label(fa_rich(&app.error_message).color(Colors::ERROR));
                    }
                });
            });
    });
}

fn nav_item(ui: &mut Ui, label: &str, active: bool) {
    let fill = if active {
        Colors::ACCENT_MUTED
    } else {
        Colors::SIDEBAR
    };
    let text_color = if active {
        Colors::ACCENT
    } else {
        Colors::TEXT_MUTED
    };

    let response = Frame::new()
        .fill(fill)
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
                ui.label(fa_rich(label).size(14.0).color(text_color));
            });
        })
        .response
        .interact(Sense::click());

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    ui.add_space(4.0);
}

fn info_card(ui: &mut Ui, title: &str, value: &str, accent: egui::Color32) {
    Frame::new()
        .fill(Colors::PANEL)
        .stroke(Stroke::new(1.0_f32, Colors::BORDER))
        .corner_radius(CornerRadius::same(14))
        .inner_margin(Margin::symmetric(18, 16))
        .show(ui, |ui| {
            ui.set_min_width(180.0);
            ui.with_layout(Layout::top_down(Align::Max).with_cross_justify(true), |ui| {
                ui.label(fa_rich(title).size(12.0).color(Colors::TEXT_MUTED));
                ui.add_space(6.0);
                rtl_row(ui, |ui| {
                    // Value may be Latin (email/username) — leave as-is if no Persian
                    let shown = if value.chars().any(fa_shaper::is_persian_char) {
                        fa(value)
                    } else {
                        value.to_string()
                    };
                    ui.label(RichText::new(shown).size(15.0).strong().color(Colors::TEXT));
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(8.0), Sense::hover());
                    ui.painter().circle_filled(rect.center(), 4.0, accent);
                });
            });
        });
    ui.add_space(10.0);
}

fn status_row(ui: &mut Ui, label: &str, value: &str, ok: bool) {
    rtl_row(ui, |ui| {
        let color = if ok { Colors::SUCCESS } else { Colors::WARNING };
        let shown = if value.chars().any(fa_shaper::is_persian_char) {
            fa(value)
        } else {
            value.to_string()
        };
        ui.label(RichText::new(shown).size(13.5).color(color));
        ui.label(fa_rich(label).size(13.5).color(Colors::TEXT_MUTED));
    });
    ui.add_space(8.0);
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n])
    }
}

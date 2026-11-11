use egui::Ui;

use super::rtl::{fa_rich, rtl_row};
use super::theme::Colors;
use super::widgets::{
    auth_shell, card, error_banner, ghost_button, labeled_text, primary_button, strength_bar,
};
use crate::app::{AuthApp, Screen};
use crate::auth::password_strength;

pub fn show_register(app: &mut AuthApp, ui: &mut Ui) {
    auth_shell(ui, 440.0, |ui| {
        card(ui, |ui| {
            ui.label(
                fa_rich("ساخت حساب جدید")
                    .size(24.0)
                    .strong()
                    .color(Colors::TEXT),
            );
            ui.add_space(4.0);
            ui.label(
                fa_rich("اطلاعات خود را وارد کنید تا حساب امن بسازید")
                    .size(13.0)
                    .color(Colors::TEXT_MUTED),
            );
            ui.add_space(12.0);

            error_banner(ui, &app.error_message);

            labeled_text(
                ui,
                "reg_display",
                "نام نمایشی",
                "مثلاً پویا",
                &mut app.register_display_name,
                false,
            );
            labeled_text(
                ui,
                "reg_user",
                "نام کاربری",
                "pouya_dev",
                &mut app.register_username,
                false,
            );
            labeled_text(
                ui,
                "reg_email",
                "ایمیل",
                "you@example.com",
                &mut app.register_email,
                false,
            );
            labeled_text(
                ui,
                "reg_pw",
                "رمز عبور",
                "حداقل ۸ کاراکتر",
                &mut app.register_password,
                true,
            );

            if !app.register_password.is_empty() {
                ui.add_space(6.0);
                strength_bar(ui, password_strength(&app.register_password));
            }

            labeled_text(
                ui,
                "reg_pw2",
                "تکرار رمز عبور",
                "••••••••",
                &mut app.register_password_confirm,
                true,
            );

            ui.add_space(14.0);
            let can_submit = !app.register_username.trim().is_empty()
                && !app.register_email.trim().is_empty()
                && !app.register_display_name.trim().is_empty()
                && !app.register_password.is_empty()
                && !app.register_password_confirm.is_empty();

            if primary_button(ui, "ثبت‌نام", can_submit && !app.busy).clicked() {
                app.submit_register();
            }

            ui.add_space(12.0);
            rtl_row(ui, |ui| {
                if ghost_button(ui, "ورود").clicked() {
                    app.clear_messages();
                    app.screen = Screen::Login;
                }
                ui.label(
                    fa_rich("قبلاً ثبت‌نام کرده‌اید؟")
                        .size(13.0)
                        .color(Colors::TEXT_MUTED),
                );
            });
        });
    });
}

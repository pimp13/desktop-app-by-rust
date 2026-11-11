use egui::Ui;

use super::rtl::{fa_rich, rtl_row};
use super::theme::Colors;
use super::widgets::{
    auth_shell, card, error_banner, ghost_button, labeled_text, primary_button, success_banner,
};
use crate::app::{AuthApp, Screen};

pub fn show_login(app: &mut AuthApp, ui: &mut Ui) {
    auth_shell(ui, 420.0, |ui| {
        card(ui, |ui| {
            ui.label(
                fa_rich("ورود به حساب")
                    .size(24.0)
                    .strong()
                    .color(Colors::TEXT),
            );
            ui.add_space(4.0);
            ui.label(
                fa_rich("نام کاربری یا ایمیل و رمز عبور خود را وارد کنید")
                    .size(13.0)
                    .color(Colors::TEXT_MUTED),
            );
            ui.add_space(16.0);

            success_banner(ui, &app.info_message);
            error_banner(ui, &app.error_message);

            let enter = labeled_text(
                ui,
                "login_id",
                "نام کاربری یا ایمیل",
                "username@example.com",
                &mut app.login_identifier,
                false,
            )
            .lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter));

            let enter2 = labeled_text(
                ui,
                "login_pw",
                "رمز عبور",
                "••••••••",
                &mut app.login_password,
                true,
            )
            .lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter));

            ui.add_space(8.0);
            rtl_row(ui, |ui| {
                ui.checkbox(
                    &mut app.remember_me,
                    fa_rich("مرا به خاطر بسپار")
                        .size(13.0)
                        .color(Colors::TEXT_MUTED),
                );
            });

            ui.add_space(14.0);
            let can_submit =
                !app.login_identifier.trim().is_empty() && !app.login_password.is_empty();

            if primary_button(ui, "ورود", can_submit && !app.busy).clicked()
                || ((enter || enter2) && can_submit)
            {
                app.submit_login();
            }

            ui.add_space(14.0);
            rtl_row(ui, |ui| {
                if ghost_button(ui, "ثبت‌نام").clicked() {
                    app.clear_messages();
                    app.screen = Screen::Register;
                }
                ui.label(
                    fa_rich("حساب ندارید؟")
                        .size(13.0)
                        .color(Colors::TEXT_MUTED),
                );
            });
        });
    });
}

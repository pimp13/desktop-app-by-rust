mod app;
mod auth;
mod ui;

use app::AuthApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([980.0, 680.0])
            .with_min_inner_size([720.0, 520.0])
            .with_title("SecureDesk — Sign in"),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "SecureDesk",
        options,
        Box::new(|cc| Ok(Box::new(AuthApp::new(cc)))),
    )
}

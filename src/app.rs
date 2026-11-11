use directories::ProjectDirs;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use std::path::PathBuf;
use std::sync::Arc;

use crate::auth::{AuthStore, Session, SessionManager};
use crate::ui::{apply_theme, show_dashboard, show_login, show_register, Colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Login,
    Register,
    Dashboard,
}

pub struct AuthApp {
    pub store: AuthStore,
    pub sessions: SessionManager,
    pub screen: Screen,
    pub session: Option<Session>,

    pub login_identifier: String,
    pub login_password: String,
    pub remember_me: bool,

    pub register_display_name: String,
    pub register_username: String,
    pub register_email: String,
    pub register_password: String,
    pub register_password_confirm: String,

    pub error_message: String,
    pub info_message: String,
    pub busy: bool,
}

impl AuthApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_theme(&cc.egui_ctx);
        load_fonts(&cc.egui_ctx);

        let data_dir = app_data_dir();
        let store = AuthStore::open(&data_dir.join("users.db"))
            .expect("failed to open auth database");
        let sessions =
            SessionManager::new(&data_dir).expect("failed to init session manager");

        let mut app = Self {
            store,
            sessions,
            screen: Screen::Login,
            session: None,
            login_identifier: String::new(),
            login_password: String::new(),
            remember_me: true,
            register_display_name: String::new(),
            register_username: String::new(),
            register_email: String::new(),
            register_password: String::new(),
            register_password_confirm: String::new(),
            error_message: String::new(),
            info_message: String::new(),
            busy: false,
        };

        match app.sessions.load() {
            Ok(Some(session)) => {
                app.session = Some(session);
                app.screen = Screen::Dashboard;
                app.info_message = "نشست قبلی شما بازیابی شد".into();
            }
            Ok(None) => {}
            Err(e) => {
                app.error_message = e.to_string();
            }
        }

        if app.session.is_none() {
            if let Ok(0) = app.store.user_count() {
                app.screen = Screen::Register;
                app.info_message = "اولین کاربر را بسازید".into();
            }
        }

        app
    }

    pub fn clear_messages(&mut self) {
        self.error_message.clear();
        self.info_message.clear();
    }

    pub fn submit_login(&mut self) {
        self.clear_messages();
        self.busy = true;

        match self
            .store
            .login(&self.login_identifier, &self.login_password)
        {
            Ok(session) => {
                if self.remember_me {
                    if let Err(e) = self.sessions.save(&session) {
                        self.error_message = e.to_string();
                    }
                } else {
                    let _ = self.sessions.clear();
                }
                self.login_password.clear();
                self.session = Some(session);
                self.screen = Screen::Dashboard;
                self.info_message = "ورود موفق بود".into();
            }
            Err(e) => {
                self.error_message = e.to_string();
            }
        }

        self.busy = false;
    }

    pub fn submit_register(&mut self) {
        self.clear_messages();

        if self.register_password != self.register_password_confirm {
            self.error_message = "رمز عبور و تکرار آن یکسان نیستند".into();
            return;
        }

        self.busy = true;
        match self.store.register(
            &self.register_username,
            &self.register_email,
            &self.register_display_name,
            &self.register_password,
        ) {
            Ok(user) => {
                self.register_password.clear();
                self.register_password_confirm.clear();
                self.login_identifier = user.username;
                self.screen = Screen::Login;
                self.info_message = "حساب ساخته شد. اکنون وارد شوید".into();
            }
            Err(e) => {
                self.error_message = e.to_string();
            }
        }
        self.busy = false;
    }

    pub fn logout(&mut self) {
        let _ = self.sessions.clear();
        self.session = None;
        self.login_password.clear();
        self.clear_messages();
        self.screen = Screen::Login;
        self.info_message = "با موفقیت خارج شدید".into();
    }

    pub fn refresh_profile(&mut self) {
        self.clear_messages();
        let Some(session) = &self.session else {
            return;
        };
        match self.store.get_user(session.user_id) {
            Ok(Some(user)) => {
                if let Some(s) = &mut self.session {
                    s.username = user.username;
                    s.email = user.email;
                    s.display_name = user.display_name;
                }
                self.info_message = "اطلاعات کاربر به‌روز شد".into();
            }
            Ok(None) => {
                self.error_message = "کاربر یافت نشد".into();
                self.logout();
            }
            Err(e) => self.error_message = e.to_string(),
        }
    }
}

impl eframe::App for AuthApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Colors::BG))
            .show(ctx, |ui| match self.screen {
                Screen::Login => show_login(self, ui),
                Screen::Register => show_register(self, ui),
                Screen::Dashboard => show_dashboard(self, ui),
            });
    }
}

fn app_data_dir() -> PathBuf {
    ProjectDirs::from("com", "securedesk", "SecureDesk")
        .map(|d| d.data_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("./data"))
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Latin first for ASCII, then Arabic/Persian (needed for reshaped presentation forms).
    if let Ok(bytes) = std::fs::read("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf") {
        fonts
            .font_data
            .insert("noto_sans".into(), Arc::new(FontData::from_owned(bytes)));
    }

    let arabic_candidates = [
        "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoNaskhArabic-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];

    for path in arabic_candidates {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("persian".into(), Arc::new(FontData::from_owned(bytes)));
            break;
        }
    }

    {
        let proportional = fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default();
        proportional.clear();
        if fonts.font_data.contains_key("noto_sans") {
            proportional.push("noto_sans".into());
        }
        if fonts.font_data.contains_key("persian") {
            proportional.push("persian".into());
        }
        proportional.push("Ubuntu-Light".into());
        proportional.push("NotoEmoji-Regular".into());
        proportional.push("emoji-icon-font".into());
    }

    {
        let mono = fonts.families.entry(FontFamily::Monospace).or_default();
        if fonts.font_data.contains_key("persian") {
            mono.push("persian".into());
        }
        if fonts.font_data.contains_key("noto_sans") {
            mono.push("noto_sans".into());
        }
    }

    ctx.set_fonts(fonts);
}

mod dashboard;
mod login;
mod register;
mod rtl;
mod theme;
mod widgets;

pub use dashboard::show_dashboard;
pub use login::show_login;
pub use register::show_register;
pub use theme::{apply_theme, Colors};

//! Persian/RTL helpers for egui (no native BiDi/shaping).
//! Reshapes Arabic/Persian letters into presentation forms and visual order.

use egui::{Align, Layout, RichText, Ui};
use fa_shaper::reshape;

/// Prepare Persian/Arabic text for correct visual display in egui.
pub fn fa(text: impl AsRef<str>) -> String {
    reshape(text.as_ref())
}

pub fn fa_rich(text: impl AsRef<str>) -> RichText {
    RichText::new(fa(text))
}

/// Run UI content in a right-aligned (RTL) column.
pub fn rtl_column(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.with_layout(Layout::top_down(Align::Max), add_contents);
}

/// Horizontal row that places items from right to left.
pub fn rtl_row(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.with_layout(Layout::right_to_left(Align::Center), add_contents);
}

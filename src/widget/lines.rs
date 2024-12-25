
use crate::{Size, UINodeParams, UI};

use super::{h_spacing, horizontal, v_spacing, vertical, Theme};

pub fn h_line(ui: &mut UI) {
    let theme = ui.style::<Theme>(); 
    let stroke_color = theme.stroke;
    let stroke_width = theme.widget_stroke_width;
    ui.node(UINodeParams::new(Size::fr(1.0), Size::px(stroke_width))
        .with_fill(stroke_color));
}

pub fn v_line(ui: &mut UI) {
    let theme = ui.style::<Theme>(); 
    let stroke_color = theme.stroke;
    let stroke_width = theme.widget_stroke_width;
    ui.node(UINodeParams::new(Size::px(stroke_width), Size::fr(1.0))
        .with_fill(stroke_color));
}

pub fn h_divider(ui: &mut UI) {
    let theme = ui.style::<Theme>();
    let margin = theme.widget_margin;
    horizontal(ui, |ui| {
        h_spacing(ui, margin);
        h_line(ui);
        h_spacing(ui, margin);
    });
}

pub fn v_divider(ui: &mut UI) {
    let theme = ui.style::<Theme>();
    let margin = theme.widget_margin;
    vertical(ui, |ui| {
        v_spacing(ui, margin);
        h_line(ui);
        v_spacing(ui, margin);
    });
}
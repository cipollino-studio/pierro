
use crate::{Axis, Margin, PerAxis, Response, Size, UINodeParams, UI};

use super::{h_spacing, horizontal, v_spacing, vertical, Theme};

fn line_params(ui: &mut UI, axis: Axis) -> UINodeParams {
    let theme = ui.style::<Theme>(); 
    let stroke_color = theme.stroke;
    let stroke_width = theme.widget_stroke_width;
    UINodeParams::new_per_axis(PerAxis::along_across(axis, Size::fr(1.0), Size::px(stroke_width)))
        .with_fill(stroke_color)
}

pub fn h_line(ui: &mut UI) {
    let params = line_params(ui, Axis::X);
    ui.node(params);
}

pub fn v_line(ui: &mut UI) {
    let params = line_params(ui, Axis::Y);
    ui.node(params);
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

const INTERACTION_MARGIN: Margin = Margin::same(5.0);

pub fn h_draggable_line(ui: &mut UI) -> Response {
    let params = line_params(ui, Axis::X)
        .with_interaction_margin(INTERACTION_MARGIN)
        .sense_mouse()
        .with_interaction_priority();
    ui.node(params)
}

pub fn v_draggable_line(ui: &mut UI) -> Response {
    let params = line_params(ui, Axis::Y)
        .with_interaction_margin(INTERACTION_MARGIN)
        .sense_mouse()
        .with_interaction_priority();
    ui.node(params)
}

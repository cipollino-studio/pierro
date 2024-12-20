
use crate::{Response, Size, UINodeParams, UI};

use super::{animate, margin, Theme};

pub fn button<S: Into<String>>(ui: &mut UI, label: S) -> Response {
    let theme = ui.style::<Theme>();
    let bg = theme.bg_button;
    let widget_margin = theme.widget_margin; 

    let response = margin(
        ui,
        widget_margin,
        UINodeParams::new(Size::fit(), Size::fit())
            .with_fill(bg)
            .sense_mouse(),
        |ui| {
            super::label(ui, label);
        }
    ).0;

    let theme = ui.style::<Theme>();
    let target_color = if response.mouse_down() {
        theme.pressed_color(bg)
    } else if response.hovered {
        theme.hovered_color(bg)
    } else {
        bg
    };
    let rate = theme.color_transition_animation_rate;
    let color = animate(ui, response.id, target_color, rate);
    ui.set_fill(response.node_ref, color);

    response
}

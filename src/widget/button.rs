
use crate::{Margin, Response, Size, UINodeParams, UI};

use super::{animate, label_text_style, Theme};

pub fn button<S: Into<String>>(ui: &mut UI, label: S) -> Response {
    let theme = ui.style::<Theme>();
    let bg = theme.bg_button;
    let margin = theme.widget_margin;
    let text_style = label_text_style(ui);

    let response = ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_fill(bg)
            .with_margin(Margin::same(margin))
            .with_text(label)
            .with_text_style(text_style)
            .sense_mouse()
    );

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

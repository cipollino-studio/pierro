
use crate::{Margin, Response, Size, TSTransform, UINodeParams, Vec2, UI};

use super::Theme;

struct ContextMenuMemory {
    position: Vec2
}

pub fn context_menu<F: FnOnce(&mut UI)>(ui: &mut UI, response: &Response, body: F) {

    if response.right_mouse_released() {
        let position = ui.input().mouse_pos.unwrap_or(Vec2::ZERO);
        ui.memory().insert(response.id, ContextMenuMemory {
            position
        });
    }

    if let Some(context_menu_memory) = ui.memory().get_opt::<ContextMenuMemory>(response.id) {
        let position = context_menu_memory.position;    
        let theme = ui.style::<Theme>();
        let fill = theme.bg_light;
        let margin = theme.widget_margin;

        let (layer, _) = ui.layer(|ui| {
            let menu = ui.node(
                UINodeParams::new(Size::fit(), Size::fit())
                    .with_fill(fill)
                    .with_margin(Margin::same(margin))
            );
            ui.with_parent(menu.node_ref, body);

            if menu.mouse_pressed_outside(ui) {
                ui.memory().remove::<ContextMenuMemory>(response.id);
            }
        });

        ui.set_transform(layer, TSTransform::translation(position));
    }

}

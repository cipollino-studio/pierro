
use crate::{Response, Size, TSTransform, UINodeParams, Vec2, UI};

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
        let fill = ui.style::<Theme>().bg_light;

        let (layer, _) = ui.layer(|ui| {
            let menu = ui.node(
                UINodeParams::new(Size::fit(), Size::fit())
                    .with_fill(fill)
            );
            ui.with_parent(menu.node_ref, body);

            if menu.mouse_pressed_outside(ui) {
                ui.memory().remove::<ContextMenuMemory>(response.id);
            }
        });

        ui.set_transform(layer, TSTransform::translation(position));
    }

}

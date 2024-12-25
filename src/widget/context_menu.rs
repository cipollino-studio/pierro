
use crate::{Id, Margin, Response, Size, TSTransform, UINodeParams, Vec2, UI};

use super::Theme;

struct ContextMenuMemory {
    position: Vec2
}

pub fn context_menu<F: FnOnce(&mut UI)>(ui: &mut UI, response: &Response, body: F) {
    if response.right_mouse_released() {
        let position = ui.input().mouse_pos.unwrap_or(Vec2::ZERO);
        open_context_menu(ui, response.id, position);        
    }

    render_context_menu(ui, response.id, body);
}

pub fn open_context_menu(ui: &mut UI, id: Id, position: Vec2) {
    ui.memory().insert(id, ContextMenuMemory {
        position
    });
}

pub fn close_context_menu(ui: &mut UI, id: Id) {
    ui.memory().remove::<ContextMenuMemory>(id);
}

pub fn is_context_menu_open(ui: &mut UI, id: Id) -> bool {
    ui.memory().has::<ContextMenuMemory>(id)
}

pub fn render_context_menu<F: FnOnce(&mut UI)>(ui: &mut UI, id: Id, body: F) {
    if let Some(context_menu_memory) = ui.memory().get_opt::<ContextMenuMemory>(id) {
        let position = context_menu_memory.position;    
        let theme = ui.style::<Theme>();
        let fill = theme.bg_light;
        let stroke = theme.widget_stroke();
        let margin = theme.widget_margin;

        let (layer, _) = ui.layer(|ui| {
            let (menu, _) = ui.with_node(
                UINodeParams::new(Size::fit(), Size::fit())
                    .with_fill(fill)
                    .with_stroke(stroke)
                    .with_margin(Margin::same(margin)),
                body
            );

            if menu.mouse_pressed_outside(ui) {
                close_context_menu(ui, id);
            }
        });

        ui.set_transform(layer, TSTransform::translation(position));
    }
}

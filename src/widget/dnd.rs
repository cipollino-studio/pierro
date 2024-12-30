
use std::any::Any;

use crate::{LayoutInfo, Response, Size, Stroke, TSTransform, UINodeParams, Vec2, UI};

use super::Theme;

struct DndSourceMemory {
    offset: Vec2,
    size: Vec2
}

pub fn dnd_source<T: Any, F: FnOnce(&mut UI)>(ui: &mut UI, payload: T, body: F) -> Response {
    let response = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .sense_mouse()
    ); 

    if response.drag_started() {
        ui.memory().set_dnd_payload(payload);
        let size = ui.memory().get::<LayoutInfo>(response.id).screen_rect.size();
        let offset = ui.memory().get::<LayoutInfo>(response.id).screen_rect.tl();
        ui.memory().insert(response.id, DndSourceMemory {
            offset,
            size
        });
        response.request_focus(ui);
    }
    if !ui.input().l_mouse.down() {
        ui.memory().remove::<DndSourceMemory>(response.id);
        response.release_focus(ui);
    }

    let drag_delta = ui.input().mouse_delta(); 
    if let Some(memory) = ui.memory().get_opt::<DndSourceMemory>(response.id) {
        memory.offset += drag_delta;
        let offset = memory.offset; 
        let size = memory.size; 
        ui.set_size(response.node_ref, Size::px(size.x), Size::px(size.y));

        let (layer, _) = ui.layer(|ui| {
            ui.with_node(UINodeParams::new(Size::px(size.x), Size::px(size.y)), body);
        });
        ui.set_transform(layer, TSTransform::translation(offset));
    } else {
        ui.with_parent(response.node_ref, body);
    }
    
    response
}

pub fn dnd_drop_zone_with_size<T: Any, F: FnOnce(&mut UI)>(ui: &mut UI, width: Size, height: Size, body: F) -> (Response, Option<T>) {
    let (response, _) = ui.with_node(
        UINodeParams::new(width, height)
            .sense_mouse(),
        body
    );

    if response.through_hovered && ui.memory().has_dnd_payload_of_type::<T>() {
        let stroke_color = ui.style::<Theme>().text_active;
        ui.set_stroke(response.node_ref, Stroke::new(stroke_color, 2.0));
    }

    (response, if response.through_hovered && ui.input().l_mouse.released() {
        ui.request_redraw();
        ui.memory().take_dnd_payload() 
    } else {
        None
    })
}

pub fn dnd_drop_zone<T: Any, F: FnOnce(&mut UI)>(ui: &mut UI, body: F) -> (Response, Option<T>) {
    dnd_drop_zone_with_size(ui, Size::fit(), Size::fit(), body)
}

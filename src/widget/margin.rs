
use crate::{Layout, Response, Size, UINodeParams, UI};

use super::{h_spacing, v_spacing};

pub fn margin<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, margin: f32, params: UINodeParams, inner: F) -> (Response, R) {
    let horiz = ui.node(
        params
            .with_size(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal())
    );

    let return_value = ui.with_parent(horiz.node_ref, |ui| {

        h_spacing(ui, margin);

        let vert = ui.node(
            UINodeParams::new(Size::fit(), Size::fit())
                .with_layout(Layout::vertical())
        );

        let return_value = ui.with_parent(vert.node_ref, |ui| {
            v_spacing(ui, margin);

            let content = ui.node(
                UINodeParams::new(Size::fit(), Size::fit())
            );

            let return_value = ui.with_parent(content.node_ref, |ui| {
                inner(ui)
            });

            v_spacing(ui, margin);

            return_value
        });

        h_spacing(ui, margin);

        return_value
    });

    (horiz, return_value)
}

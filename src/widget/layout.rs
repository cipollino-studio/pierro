
use crate::{Layout, Response, Size, UINodeParams, UI};

pub fn horizontal<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fr(1.0), Size::fit())
            .with_layout(Layout::horizontal())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn horizontal_fit<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn vertical<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fr(1.0), Size::fit())
            .with_layout(Layout::vertical())
    );
    (container, ui.with_parent(container.node_ref, body))
}

pub fn vertical_fit<R, F: FnOnce(&mut UI) -> R>(ui: &mut UI, body: F) -> (Response, R) {
    let container = ui.node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::vertical())
    );
    (container, ui.with_parent(container.node_ref, body))
}

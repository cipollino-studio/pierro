
use std::collections::HashMap;

use crate::{TSTransform, Vec2};

use super::{Id, LayoutMemory, Memory, UIRef, UI};

mod key;
pub use key::*;

/// The state of an input device that can either be down or not.
/// Has utilities for detecting the frame the button was first pressed, released, etc.
#[derive(Clone, Copy)]
pub struct ButtonInput {
    state: bool,
    prev_state: bool
}

impl ButtonInput {

    pub fn new() -> Self {
        Self {
            state: false,
            prev_state: false,
        }
    }

    /// Update the button with a new state
    pub fn tick(&mut self, state: bool) {
        self.prev_state = self.state;
        self.state = state;
    } 

    /// Set the button to be down 
    pub fn press(&mut self) {
        self.state = true;
    }

    /// Set the button to be up
    pub fn release(&mut self) {
        self.state = false;
    }

    /// Update the button without providing a new state
    pub fn tick_with_same_state(&mut self) {
        self.prev_state = self.state;
    }

    /// Is the button down?
    pub fn down(&self) -> bool {
        self.state
    }

    /// Has the button just been pressed?
    pub fn pressed(&self) -> bool {
        self.state && !self.prev_state
    } 

    /// Has the button just been released?
    pub fn released(&self) -> bool {
        !self.state && self.prev_state
    }
    
}

/// The raw input given to the application by the windowing library
pub(crate) struct RawInput {
    /// Mouse position in physical pixels. None if the mouse left the window
    pub(crate) mouse_pos: Option<Vec2>,
    /// Is the mouse currently down?
    pub(crate) mouse_down: bool,
    /// How much has the mouse scrolled
    pub(crate) scroll: Vec2,

    /// What keys were pressed this frame?
    pub(crate) keys_pressed: Vec<Key>,
    /// What keys were released this frame?
    pub(crate) keys_released: Vec<Key>
}

impl RawInput {

    pub(crate) fn new() -> Self {
        Self {
            mouse_pos: None,
            mouse_down: false,
            scroll: Vec2::ZERO,
            keys_pressed: Vec::new(),
            keys_released: Vec::new()
        }
    }

}

pub struct Input {
    pub prev_mouse_pos: Option<Vec2>,
    pub mouse_pos: Option<Vec2>,
    pub l_mouse: ButtonInput,
    pub l_mouse_press_pos: Option<Vec2>,
    pub dragging: ButtonInput,
    pub scroll: Vec2,

    keys: HashMap<Key, ButtonInput>,
    pub keys_pressed: Vec<Key>,
    pub keys_released: Vec<Key>
}

/// The memory storing what inputs are provided to a node
pub(crate) struct Interaction {
    pub(crate) hovered: bool,
    pub(crate) l_mouse: ButtonInput,
    pub(crate) dragging: ButtonInput,
    pub(crate) scroll: Vec2
}

impl Default for Interaction {

    fn default() -> Self {
        Self {
            hovered: false,
            l_mouse: ButtonInput::new(),
            dragging: ButtonInput::new(),
            scroll: Vec2::ZERO
        }
    }

}

/// Find the node hovered at a given position in screen space
fn find_hover_node(memory: &mut Memory, node: Id, pos: Vec2) -> Option<Id> {
    let mut child = memory.get::<LayoutMemory>(node).first_child;
    while let Some(child_id) = child {
        if let Some(node) = find_hover_node(memory, child_id, pos) {
            return Some(node);
        }
        child = memory.get::<LayoutMemory>(child_id).next;
    } 

    let layout_mem = memory.get::<LayoutMemory>(node);
    if layout_mem.screen_rect.contains(pos) && layout_mem.sense_mouse {
        return Some(node);
    }

    None
}

impl Input {

    /// The change in mouse position between the previous and current frame
    pub fn mouse_delta(&self) -> Vec2 {
        let Some(mouse_pos) = self.mouse_pos else { return Vec2::ZERO };
        let Some(prev_mouse_pos) = self.prev_mouse_pos else { return Vec2::ZERO };
        mouse_pos - prev_mouse_pos
    }

    /// Get the state of a key
    pub fn key_state(&self, key: Key) -> ButtonInput {
        self.keys.get(&key).map(|state| *state).unwrap_or(ButtonInput::new())
    }

    /// Is a key down?
    pub fn key_down(&self, key: Key) -> bool {
        self.key_state(key).down()
    }

    /// Has a key just been pressed?
    pub fn key_pressed(&self, key: Key) -> bool {
        self.key_state(key).pressed()
    }

    /// Has a key just been released?
    pub fn key_released(&self, key: Key) -> bool {
        self.key_state(key).released()
    }

    /// Get a mutable reference to the state of a key
    fn key_state_mut(&mut self, key: &Key) -> &mut ButtonInput {
        if !self.keys.contains_key(&key) {
            self.keys.insert(key.clone(), ButtonInput::new());
        }
        self.keys.get_mut(&key).unwrap()
    }

    pub(crate) fn new() -> Self {
        Self {
            prev_mouse_pos: None,
            mouse_pos: None,
            l_mouse: ButtonInput::new(),
            l_mouse_press_pos: None,
            dragging: ButtonInput::new(),
            scroll: Vec2::ZERO,
            keys: HashMap::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new()
        }
    }

    /// Update the input given the raw input from the window.
    /// Resets the raw input in preparation for the next frame.
    pub(crate) fn update(&mut self, raw_input: &mut RawInput, scale_factor: f32) {
        self.prev_mouse_pos = self.mouse_pos;
        self.mouse_pos = raw_input.mouse_pos.map(|pos| pos / scale_factor);
        self.l_mouse.tick(raw_input.mouse_down);
        self.dragging.tick_with_same_state();
        if self.l_mouse.pressed() {
            self.l_mouse_press_pos = self.mouse_pos;
        }
        if self.l_mouse.down() {
            if self.mouse_pos.unwrap_or(Vec2::INFINITY).distance(self.l_mouse_press_pos.unwrap_or(Vec2::INFINITY)) > 2.5 {
                self.dragging.press();
            }
        }
        if self.l_mouse.released() {
            self.l_mouse_press_pos = None;
            self.dragging.release();
        }
        self.scroll = raw_input.scroll / scale_factor;
        raw_input.scroll = Vec2::ZERO;

        self.keys_pressed = std::mem::replace(&mut raw_input.keys_pressed, Vec::new());
        self.keys_released = std::mem::replace(&mut raw_input.keys_released, Vec::new());
        for key in self.keys_pressed.clone() {
            self.key_state_mut(&key).press();
        }
        for key in self.keys_released.clone() {
            self.key_state_mut(&key).release();
        }

        for (_key, state) in self.keys.iter_mut() {
            state.tick_with_same_state();
        }

    }

    /// Distribute the input to nodes, taking foucs into account.
    pub(crate) fn distribute(&self, root: Id, memory: &mut Memory) {
        let hovered_node = memory.get_focus().or_else(|| self.mouse_pos.map(|pos| find_hover_node(memory, root, pos)).flatten());

        for (id, interaction) in memory.iter_mut::<Interaction>() {
            let hovered = Some(id) == hovered_node;
            interaction.hovered = hovered;
            interaction.l_mouse = if hovered { self.l_mouse } else { ButtonInput::new() };
            interaction.dragging = if hovered { self.dragging } else { ButtonInput::new() };
            interaction.scroll = if hovered { self.scroll } else { Vec2::ZERO };
        }
    }

}

#[derive(Clone, Copy)]
pub struct Response {
    pub id: Id,
    pub node_ref: UIRef,

    pub hovered: bool,
    pub l_mouse: ButtonInput,
    pub dragging: ButtonInput,
    pub scroll: Vec2
}

impl Response {

    pub fn contains_mouse(&self, ui: &mut UI) -> bool {
        let Some(pos) = ui.input().mouse_pos else { return false; };
        ui.memory().get::<LayoutMemory>(self.id).screen_rect.contains(pos)
    }

    /// Returns the position of the mouse relative to the node
    pub fn mouse_pos(&self, ui: &mut UI) -> Option<Vec2> {
        let screen_pos = ui.input().mouse_pos?;
        let layout_memory = ui.memory().get::<LayoutMemory>(self.id);
        let rect = layout_memory.screen_rect;
        let scale = layout_memory.transform.scale;
        Some((screen_pos - rect.tl()) / scale) 
    }

    pub fn mouse_down(&self) -> bool {
        self.l_mouse.down()
    }

    pub fn mouse_pressed(&self) -> bool {
        self.l_mouse.pressed()
    }

    pub fn mouse_released(&self) -> bool {
        self.l_mouse.released()
    }

    pub fn mouse_pressed_outside(&self, ui: &mut UI) -> bool {
        self.mouse_pressed() && !self.contains_mouse(ui)
    }

    pub fn dragging(&self) -> bool {
        self.dragging.down()
    }

    pub fn drag_started(&self) -> bool {
        self.dragging.pressed()
    }

    pub fn drag_stopped(&self) -> bool {
        self.dragging.released()
    }

    pub fn drag_delta(&self, ui: &mut UI) -> Vec2 {
        if !self.dragging() {
            return Vec2::ZERO;
        }
        let scale = self.scale(ui);
        ui.input().mouse_delta() / scale
    }

    pub fn is_focused(&self, ui: &mut UI) -> bool {
        ui.memory().is_focused(self.id)
    }

    pub fn request_focus(&self, ui: &mut UI) {
        ui.memory().request_focus(self.id);
    }

    pub fn release_focus(&self, ui: &mut UI) {
        if self.is_focused(ui) {
            ui.memory().release_focus();
        }
    }

    pub fn transform(&self, ui: &mut UI) -> TSTransform {
        ui.memory().get::<LayoutMemory>(self.id).transform
    }

    pub fn scale(&self, ui: &mut UI) -> f32 {
        self.transform(ui).scale
    }

}

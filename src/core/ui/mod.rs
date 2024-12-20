
pub mod layout;
use std::{any::Any, fmt::Debug};

pub use layout::*;

pub mod memory;
pub use memory::*;

pub mod style;
use style::*;

pub mod input;
pub use input::*;

mod paint;

use crate::{Axis, Color, PerAxis, Rect, Vec2};

use super::{Painter, RenderResources, TSTransform, TextStyle};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UIRef {
    Null,
    Some(usize)
}

impl Debug for UIRef {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Some(idx) => f.debug_tuple("some").field(idx).finish(),
        }
    }

}

impl UIRef {

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn as_option(&self) -> Option<Self> {
        match self {
            UIRef::Null => None,
            UIRef::Some(_) => Some(*self),
        }
    } 

}

pub struct UINodeParams {
    // Layout
    pub(crate) size: PerAxis<Size>,
    pub(crate) layout: Layout,
    pub(crate) transform: TSTransform,

    // Styling
    pub(crate) fill: Option<Color>,

    // Text
    pub(crate) text: Option<String>,
    pub(crate) text_style: TextStyle,

    // Id
    pub(crate) id_source: Option<u64>,

    // Input
    pub(crate) mouse: bool,

    // Custom Behaviour 
    pub(crate) on_paint: Option<Box<dyn FnOnce(&mut Painter, Rect)>>
}

impl UINodeParams {

    pub fn new(w: Size, h: Size) -> Self {
        Self {
            size: PerAxis::new(w, h),
            layout: Layout::new(Axis::Y),
            transform: TSTransform::IDENTITY,
            fill: None,
            text: None,
            text_style: TextStyle::default(),
            id_source: None,
            mouse: false,
            on_paint: None
        }
    }

    pub fn with_size(mut self, w: Size, h: Size) -> Self {
        self.size.x = w;
        self.size.y = h;
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_transform(mut self, transform: TSTransform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    pub fn sense_mouse(mut self) -> Self {
        self.mouse = true;
        self
    }

    pub fn on_paint<F: FnOnce(&mut Painter, Rect) + 'static>(mut self, on_paint: F) -> Self {
        self.on_paint = Some(Box::new(on_paint));
        self
    }

}

pub(crate) struct UINode {
    pub(crate) id: Id,

    // tree links
    pub(crate) next: UIRef,
    pub(crate) prev: UIRef,
    pub(crate) first_child: UIRef,
    pub(crate) last_child: UIRef,
    pub(crate) n_children: u64,
    pub(crate) parent: UIRef,

    pub(crate) params: UINodeParams,

    // layout
    pub(crate) rect: Rect,
    pub(crate) transform: TSTransform,
    pub(crate) basis_size: Vec2,
    pub(crate) frac_units: Vec2
}

impl UINode {

    pub(crate) fn new(parent_id: Id, sibling_idx: u64, params: UINodeParams) -> Self {
        Self {
            id: Id(ahash::RandomState::with_seeds(1, 9, 8, 4).hash_one((parent_id, params.id_source.unwrap_or(sibling_idx)))),
            next: UIRef::Null,
            prev: UIRef::Null,
            first_child: UIRef::Null,
            last_child: UIRef::Null,
            n_children: 0,
            parent: UIRef::Null,
            params,
            rect: Rect::ZERO,
            transform: TSTransform::IDENTITY,
            basis_size: Vec2::ZERO,
            frac_units: Vec2::ONE
        }
    }

}

/// A tree of UI nodes
pub(crate) struct UITree {
    /// All the nodes in the tree
    nodes: Vec<UINode>,
    /// The root node of each layer of the UI.
    /// Layers cover the entire screen and are drawn in order, allowing for popups, context menus, etc.
    /// Each layer is its own tree of nodes, with the layer node being the root.
    layers: Vec<UIRef>
}

impl UITree {

    pub(crate) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            layers: Vec::new()
        }
    }

    pub(crate) fn get(&self, node: UIRef) -> &UINode {
        match node {
            UIRef::Null => panic!("cannot get null node ref."),
            UIRef::Some(idx) => &self.nodes[idx],
        } 
    }

    pub(crate) fn get_mut(&mut self, node: UIRef) -> &mut UINode {
        match node {
            UIRef::Null => panic!("cannot get null node ref."),
            UIRef::Some(idx) => &mut self.nodes[idx],
        } 
    }

    pub(crate) fn add_node(&mut self, node: UINode) -> UIRef {
        self.nodes.push(node); 
        UIRef::Some(self.nodes.len() - 1)
    }
    
    pub(crate) fn add_layer(&mut self, size: Vec2) -> UIRef {
        let layer = self.add_node(UINode::new(
            Id(0),
            self.layers.len() as u64,
            UINodeParams::new(Size::px(size.x), Size::px(size.y)) 
        ));
        self.layers.push(layer);
        layer
    }

}

pub struct UI<'a, 'b> {
    input: &'a Input,
    memory: &'a mut Memory,
    style: Style,

    render_resources: &'a mut RenderResources<'b>,

    window_size: Vec2,

    // tree-building
    tree: UITree,
    parent_stack: Vec<UIRef>,
    curr_sibling: UIRef,

    // communication
    pub(crate) request_redraw: bool
}

impl<'a, 'b> UI<'a, 'b> {

    pub(crate) fn new(input: &'a Input, memory: &'a mut Memory, render_resources: &'a mut RenderResources<'b>, window_size: Vec2, tree: UITree, layer: UIRef) -> Self {
        Self {
            input,
            memory,
            style: Style::new(),
            render_resources,
            window_size,
            tree,
            parent_stack: vec![layer],
            curr_sibling: UIRef::Null,
            request_redraw: false
        }
    }

    pub(crate) fn tree(self) -> UITree {
        self.tree
    }

    pub fn curr_parent(&self) -> UIRef {
        let Some(parent) = self.parent_stack.last() else { panic!("no parents in parent stack. ui in an invalid state.") };
        *parent
    }

    pub fn node(&mut self, params: UINodeParams) -> Response {
        let parent_ref = self.curr_parent();
        let parent = self.tree.get(parent_ref);

        let mut new_node = UINode::new(parent.id, parent.n_children, params);
        new_node.prev = self.curr_sibling;
        new_node.parent = parent_ref;

        let new_node = self.tree.add_node(new_node);

        match self.curr_sibling {
            UIRef::Null => {
                self.tree.get_mut(parent_ref).first_child = new_node;
            },
            UIRef::Some(_) => {
                self.tree.get_mut(self.curr_sibling).next = new_node;
            },
        }

        self.curr_sibling = new_node;
        self.tree.get_mut(parent_ref).last_child = new_node;
        self.tree.get_mut(parent_ref).n_children += 1; 

        let id = self.tree.get(new_node).id;
        let interaction = self.memory.get::<Interaction>(id);
        
        Response {
            id,
            node_ref: new_node,
            hovered: interaction.hovered,
            l_mouse: interaction.l_mouse,
            r_mouse: interaction.r_mouse,
            scroll: interaction.scroll
        }
    }

    pub(crate) fn push_parent(&mut self, parent: UIRef) {
        self.parent_stack.push(parent);
        self.curr_sibling = self.tree.get(parent).last_child;
    }

    pub(crate) fn pop_parent(&mut self) {
        self.parent_stack.pop();
        if self.parent_stack.is_empty() {
            panic!("ui parent stack underflow!");
        }
        self.curr_sibling = self.tree.get(self.curr_parent()).last_child;
    }

    pub fn with_parent<R, F: FnOnce(&mut Self) -> R>(&mut self, parent: UIRef, body: F) -> R {
        self.push_parent(parent);
        let body_result = body(self);
        self.pop_parent();
        body_result
    }

    pub fn layer<R, F: FnOnce(&mut Self) -> R>(&mut self, body: F) -> (UIRef, R) {
        let layer = self.tree.add_layer(self.window_size);
        (layer, self.with_parent(layer, body))
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn style<T: Default + Any>(&mut self) -> &T {
        self.style.get()
    }

    pub fn with_style<T: Default + Any, F: FnOnce(&mut Self)>(&mut self, style: T, body: F) {
        self.style.push(style);
        body(self);
        self.style.pop();
    }

    pub fn set_fill(&mut self, node: UIRef, fill: Color) {
        self.tree.get_mut(node).params.fill = Some(fill);
    }

    pub fn set_transform(&mut self, node: UIRef, transform: TSTransform) {
        self.tree.get_mut(node).params.transform = transform;
    }

    pub fn set_text<S: Into<String>>(&mut self, node: UIRef, text: S) {
        self.tree.get_mut(node).params.text = Some(text.into());
    }
    
    pub fn set_on_paint<F: FnOnce(&mut Painter, Rect) + 'static>(&mut self, node: UIRef, on_paint: F) {
        self.tree.get_mut(node).params.on_paint = Some(Box::new(on_paint));
    }

    pub fn request_redraw(&mut self) {
        self.request_redraw = true;
    }

    /// Get the WebGPU render device
    pub fn wgpu_device(&mut self) -> &wgpu::Device {
        &self.render_resources.device
    } 

    /// Get the WebGPU render queue
    pub fn wgpu_queue(&mut self) -> &wgpu::Queue {
        &self.render_resources.queue
    }

    /// Get the COSMIC Text font system
    pub fn font_system(&mut self) -> &mut cosmic_text::FontSystem {
        &mut self.render_resources.text_resources.font_system
    }

}

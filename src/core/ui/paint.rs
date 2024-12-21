
use crate::{PaintRect, PaintText, Painter};

use super::{UIRef, UITree};

impl UITree {

    fn paint_node(&mut self, painter: &mut Painter, node_ref: UIRef) {

        let node = self.get_mut(node_ref); 

        painter.push_transform(node.transform);
        painter.push_clip_rect(node.rect);

        if let Some(color) = node.params.fill {
            painter.rect(PaintRect::new(node.rect, color));
        }

        if let Some(text) = node.params.text.take() {
            let text_rect = node.params.margin.apply(node.rect);
            painter.text(PaintText::new(text, node.params.text_style, text_rect));
        }

        let mut child = node.first_child;
        while child.is_some() {
            self.paint_node(painter, child);
            child = self.get(child).next;
        }

        let node = self.get_mut(node_ref); 
        if let Some(on_paint) = node.params.on_paint.take() {
            on_paint(painter, node.rect);
        } 

        painter.pop_clip_rect();
        painter.pop_transform();
    }

    pub(crate) fn paint(&mut self, painter: &mut Painter) {
        for layer in self.layers.clone() {
            self.paint_node(painter, layer);
        }
    }

}

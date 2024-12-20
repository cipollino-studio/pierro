
use crate::{PaintRect, PaintText, Painter};

use super::{UIRef, UITree};

impl UITree {

    pub(crate) fn paint(&mut self, painter: &mut Painter, node_ref: UIRef) {

        let node = self.get_mut(node_ref); 

        painter.push_transform(node.transform);
        painter.push_clip_rect(node.rect);

        if let Some(color) = node.params.fill {
            painter.rect(PaintRect::new(node.rect, color));
        }

        if let Some(text) = node.params.text.take() {
            painter.text(PaintText::new(text, node.params.text_style, node.rect));
        }

        let mut child = node.first_child;
        while child.is_some() {
            self.paint(painter, child);
            child = self.get(child).next;
        }

        let node = self.get_mut(node_ref); 
        if let Some(on_paint) = node.params.on_paint.take() {
            on_paint(painter, node.rect);
        } 

        painter.pop_clip_rect();
        painter.pop_transform();
    }

}

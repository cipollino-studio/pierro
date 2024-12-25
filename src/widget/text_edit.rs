

use cosmic_text::Edit;

use crate::{vec2, Key, LayoutInfo, LogicalKey, PaintRect, PaintText, Rect, Size, TextStyle, UINodeParams, Vec2, UI};

use super::Theme;

struct TextEditMemory {
    editor: cosmic_text::Editor<'static>,
    scroll: f32
}

pub fn text_edit(ui: &mut UI, text: &mut String) {

    let theme = ui.style::<Theme>();
    let color = theme.bg_text_field;
    let widget_margin = theme.widget_margin;
    let widget_rounding = theme.widget_rounding;
    let font_size = theme.label_font_size;
    let font_color = theme.text;

    let size = 200.0;

    let text_edit = ui.node(
        UINodeParams::new(Size::px(size), Size::px(font_size + 2.0 * widget_margin))
            .sense_mouse()
            .with_fill(color)
            .with_rounding(widget_rounding)
    );

    if text_edit.mouse_pressed() && !text_edit.is_focused(ui) {
        text_edit.request_focus(ui);
        let mut buffer = cosmic_text::Buffer::new(ui.font_system(), cosmic_text::Metrics { font_size, line_height: font_size });
        buffer.set_text(ui.font_system(), text, cosmic_text::Attrs::new().family(cosmic_text::Family::SansSerif), cosmic_text::Shaping::Advanced);
        let editor = cosmic_text::Editor::new(buffer);
        ui.memory().insert(text_edit.id, TextEditMemory {
            editor,
            scroll: 0.0
        });
    }
    if text_edit.mouse_pressed_outside(ui) {
        text_edit.release_focus(ui);
    }
    if !text_edit.is_focused(ui) {
        ui.memory().remove::<TextEditMemory>(text_edit.id);
    }

    let focused = text_edit.is_focused(ui); 
    let theme = ui.style::<Theme>();
    let target_color = if focused {
        theme.pressed_color(color)
    } else if text_edit.hovered {
        theme.hovered_color(color)
    } else {
        color
    };

    ui.set_fill(text_edit.node_ref, target_color);

    if let Some(mut memory) = ui.memory().remove::<TextEditMemory>(text_edit.id) {

        // Keyboard input
        for key in ui.input().keys_pressed.clone() {
            if let Some(text) = key.text {
                for char in text.chars() {
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Insert(char));
                }
            }
            match key.logical_key {
                Some(LogicalKey::Space) => {
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Insert(' '));
                },
                Some(LogicalKey::ArrowLeft) => {
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Motion(cosmic_text::Motion::Left));
                },
                Some(LogicalKey::ArrowRight) => {
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Motion(cosmic_text::Motion::Right));
                },
                Some(LogicalKey::Backspace) => {
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Backspace);
                },
                _ => {}
            }
        }

        // Update text
        memory.editor.with_buffer(|buffer| {
            let buffer_text = buffer.lines.first()?.text();
            *text = buffer_text.to_string();
            Some(())
        });
        memory.editor.shape_as_needed(ui.font_system(), true);

        // Update scroll
        let cursor_pos = memory.editor.cursor_position();
        let text_edit_width = ui.memory().get::<LayoutInfo>(text_edit.id).rect.size().x;
        if let Some((cursor_x, _)) = cursor_pos {
            memory.scroll = memory.scroll.max(cursor_x as f32 - text_edit_width + 10.0);
            memory.scroll = memory.scroll.min(cursor_x as f32);
        }
        let scroll = memory.scroll;

        // Mouse interactions
        if text_edit.mouse_pressed() {
            if let Some(mouse_pos) = text_edit.mouse_pos(ui) {
                let mouse_pos = mouse_pos - Vec2::splat(widget_margin);
                if !ui.input().key_down(Key::SHIFT) {
                    memory.editor.set_selection(cosmic_text::Selection::None);
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Click { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
                } else {
                    memory.editor.action(ui.font_system(), cosmic_text::Action::Drag { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
                }
            }
        }
        if text_edit.dragging() {
            if let Some(mouse_pos) = text_edit.mouse_pos(ui) {
                let mouse_pos = mouse_pos - Vec2::splat(widget_margin);
                memory.editor.action(ui.font_system(), cosmic_text::Action::Drag { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
            }
        }

        // Paint text, cursor and selection
        let paint_text = text.clone();
        let ranges = memory.editor.selection_bounds().map(|(from, to)| {
            let from = from.index;
            let to = to.index;
            memory.editor.with_buffer(|buffer| {
                highlight_line(&buffer.lines[0], from, to).collect::<Vec<_>>()
            })
        }).unwrap_or_default();

        ui.set_on_paint(text_edit.node_ref, move |painter, rect| {
            painter.text(PaintText::new(paint_text, TextStyle {
                color: font_color,
                font_size,
                line_height: 1.0,
            }, Rect::to_infinity(rect.tl() + Vec2::splat(widget_margin) - Vec2::X * scroll)));

            let origin = rect.tl() + Vec2::splat(widget_margin);
            if let Some((cursor_x, cursor_y)) = cursor_pos {
                let cursor_rect = Rect::min_size(
                    origin + vec2(cursor_x as f32 - scroll, cursor_y as f32),
                    vec2(1.0, font_size)
                );
                painter.rect(PaintRect::new(cursor_rect, font_color));
            }

            for (from_x, width) in ranges {
                painter.rect(PaintRect::new(
                    Rect::min_size(
                        rect.tl() + vec2(from_x - scroll + widget_margin, widget_margin),
                        vec2(width, font_size) 
                    ),
                    font_color.with_alpha(0.2))
                );
            }
            
        });

        // Put the memory back where it belongs
        ui.memory().insert(text_edit.id, memory);
    } else {

        // Paint text
        let paint_text = text.clone();
        ui.set_on_paint(text_edit.node_ref, move |painter, rect| {
            painter.text(PaintText::new(paint_text, TextStyle {
                color: font_color,
                font_size,
                line_height: 1.0,
            }, Rect::to_infinity(rect.tl() + Vec2::splat(widget_margin))));
        });

    }

}

// Taken from iced.
// TODO: proper bidi text selection
fn highlight_line(
    line: &cosmic_text::BufferLine,
    from: usize,
    to: usize,
) -> impl Iterator<Item = (f32, f32)> + '_ {
    let layout = line
        .layout_opt()
        .as_ref()
        .map(Vec::as_slice)
        .unwrap_or_default();

    layout.iter().map(move |visual_line| {
        let start = visual_line
            .glyphs
            .first()
            .map(|glyph| glyph.start)
            .unwrap_or(0);
        let end = visual_line
            .glyphs
            .last()
            .map(|glyph| glyph.end)
            .unwrap_or(0);

        let range = start.max(from)..end.min(to);

        if range.is_empty() {
            (0.0, 0.0)
        } else if range.start == start && range.end == end {
            (0.0, visual_line.w)
        } else {
            let first_glyph = visual_line
                .glyphs
                .iter()
                .position(|glyph| range.start <= glyph.start)
                .unwrap_or(0);

            let mut glyphs = visual_line.glyphs.iter();

            let x =
                glyphs.by_ref().take(first_glyph).map(|glyph| glyph.w).sum();

            let width: f32 = glyphs
                .take_while(|glyph| range.end > glyph.start)
                .map(|glyph| glyph.w)
                .sum();

            (x, width)
        }
    })
}
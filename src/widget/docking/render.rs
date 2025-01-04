
use crate::{button_fill_animation, dnd_drop_zone_with_size, dnd_source, h_draggable_line, horizontal_fit, icon_text_style, icons, left_click_context_menu, menu_bar, tab, v_draggable_line, v_line, Axis, Color, CursorIcon, Layout, LayoutInfo, Margin, PaintRect, PerAxis, ScrollArea, Size, Stroke, Theme, UINodeParams, UI};

use super::{command::{DockingCommand, TabDragSource}, DockingNodeId, DockingNodeKind, DockingState, DockingTab, DockingTree, Tabs};

impl<Tab: DockingTab> Tabs<Tab> {

    fn render_tab(&mut self, ui: &mut UI, node_id: DockingNodeId, tab_idx: usize, commands: &mut Vec<DockingCommand<Tab>>) {
        let selected = self.active_tab == tab_idx;
        let theme = ui.style::<Theme>();
        let base_color = if selected { theme.bg_light } else { theme.bg_dark };
        let (dnd_response, tab_response) = dnd_source(ui, TabDragSource { node_id, tab_idx }, |ui| {
            let tab_response = tab(ui, self.tabs[tab_idx].title(), selected);
            if tab_response.close_button.mouse_released() {
                commands.push(DockingCommand::CloseTab { tab: TabDragSource { node_id, tab_idx } });
            }
            ui.set_sense_mouse(tab_response.tab.node_ref, false);
            tab_response.tab
        });
        if dnd_response.mouse_released() {
            self.active_tab = tab_idx;
        }
        button_fill_animation(ui, tab_response.node_ref, &dnd_response, base_color); 
    }

    fn render(&mut self, ui: &mut UI, node_id: DockingNodeId, commands: &mut Vec<DockingCommand<Tab>>) {

        let theme = ui.style::<Theme>();
        let window_bg = theme.bg_light;
        let margin = theme.widget_margin;
        let split_overlay_stroke_color = theme.text_active;

        menu_bar(ui, |ui| {

            ScrollArea::default()
                .hide_scroll_bars()
                .with_size(Size::fit(), Size::fr(1.0))
                .render(ui, |ui| {
                    horizontal_fit(ui, |ui| { 
                        for tab_idx in 0..self.tabs.len() {
                            self.render_tab(ui, node_id, tab_idx, commands);
                            v_line(ui);
                        }
                    });
            });

            let (_, added_tab) = dnd_drop_zone_with_size::<TabDragSource, _>(ui, Size::fr(1.0), Size::fr(1.0), |_| {});
            if let Some(added_tab) = added_tab {
                commands.push(DockingCommand::MoveTab { from: added_tab, to: node_id });
            }

            v_line(ui);

            let icon_text_style = icon_text_style(ui);
            let add_tab_button = ui.node(
                UINodeParams::new(Size::text().no_shrink(), Size::text())
                    .with_text(icons::PLUS)
                    .with_text_style(icon_text_style)
                    .with_margin(Margin::same(margin))
                    .sense_mouse()
            );
            button_fill_animation(ui, add_tab_button.node_ref, &add_tab_button, window_bg);

            left_click_context_menu(ui, &add_tab_button, |ui| {
                Tab::add_tab_dropdown(ui, |tab| {
                    commands.push(DockingCommand::AddTab { tab, to: node_id });
                }); 
            });
        });
        
        let response = ui.node(
            UINodeParams::new(Size::fr(1.0), Size::fr(1.0))
                .with_fill(window_bg)
        );

        // Splitting
        let mut split_left = false;
        let mut split_right = false;
        let mut split_up = false;
        let mut split_down = false;
        if ui.memory().has_dnd_payload_of_type::<TabDragSource>() {
            if let Some(mouse_pos) = ui.input().mouse_pos {
                let rect = ui.memory().get::<LayoutInfo>(response.id).screen_rect;
                if rect.contains(mouse_pos) {
                    let delta = mouse_pos - rect.center(); 
                    let h_split = delta.abs().max_axis() == Axis::X;
                    split_left = h_split && delta.x < 0.0; 
                    split_right = h_split && delta.x > 0.0;
                    split_up = !h_split && delta.y < 0.0;
                    split_down = !h_split && delta.y > 0.0;
                    ui.set_on_paint(response.node_ref, move |painter, rect| {
                        let stroke = Stroke::new(split_overlay_stroke_color, 2.0); 
                        if split_left {
                            painter.rect(PaintRect::new(rect.left_half(), Color::TRANSPARENT).with_stroke(stroke));
                        }
                        if split_right {
                            painter.rect(PaintRect::new(rect.right_half(), Color::TRANSPARENT).with_stroke(stroke));
                        }
                        if split_up {
                            painter.rect(PaintRect::new(rect.top_half(), Color::TRANSPARENT).with_stroke(stroke));
                        }
                        if split_down {
                            painter.rect(PaintRect::new(rect.bottom_half(), Color::TRANSPARENT).with_stroke(stroke));
                        }
                    });
                }
            }
        }
        if ui.input().l_mouse.released() {
            if split_left {
                if let Some(tab) = ui.memory().take_dnd_payload::<TabDragSource>() {
                    commands.push(DockingCommand::Split { tab, to: node_id, direction: Axis::X, max: false });
                }
            }
            if split_right {
                if let Some(tab) = ui.memory().take_dnd_payload::<TabDragSource>() {
                    commands.push(DockingCommand::Split { tab, to: node_id, direction: Axis::X, max: true });
                }
            }
            if split_up {
                if let Some(tab) = ui.memory().take_dnd_payload::<TabDragSource>() {
                    commands.push(DockingCommand::Split { tab, to: node_id, direction: Axis::Y, max: false });
                }
            }
            if split_down {
                if let Some(tab) = ui.memory().take_dnd_payload::<TabDragSource>() {
                    commands.push(DockingCommand::Split { tab, to: node_id, direction: Axis::Y, max: true });
                }
            }
        }

        if self.tabs.len() > 0 {
            self.active_tab = self.active_tab.min(self.tabs.len() - 1);
            ui.with_parent(response.node_ref, |ui| {
                self.tabs[self.active_tab].render(ui);
            })
        }

    }

}

impl<Tab: DockingTab> DockingTree<Tab> {

    fn render_node(&mut self, ui: &mut UI, node_id: DockingNodeId, commands: &mut Vec<DockingCommand<Tab>>) -> Option<()> {
        let node = self.nodes.get_mut(&node_id)?;
        match &mut node.kind {
            DockingNodeKind::Tabs(tabs) => {
                tabs.render(ui, node_id, commands);
            },
            DockingNodeKind::Split(split) => {
                let nodes = split.nodes.clone();
                let direction = split.direction;
                let total_splits_size: f32 = nodes.iter().map(|(size, _)| *size).sum();
                let response = ui.node(
                    UINodeParams::new(Size::fr(1.0), Size::fr(1.0))
                        .with_layout(Layout::new(direction))
                );
                let size = ui.memory().get::<LayoutInfo>(response.id).rect.size().on_axis(direction);
                ui.with_parent(response.node_ref, |ui| {
                    for i in 0..nodes.len() {
                        ui.with_node(
                            UINodeParams::new_per_axis(PerAxis::along_across(direction, Size::fr(nodes[i].0), Size::fr(1.0))),
                            |ui| {
                                self.render_node(ui, nodes[i].1, commands);
                            }
                        );
                        if i < nodes.len() - 1 {
                            let response = match direction {
                                Axis::X => v_draggable_line(ui),
                                Axis::Y => h_draggable_line(ui),
                            };
                            if response.drag_started() {
                                response.request_focus(ui);
                            }
                            if response.drag_stopped() {
                                response.release_focus(ui);
                            }
                            if response.dragging() {
                                let drag = response.drag_delta(ui).on_axis(direction);
                                commands.push(DockingCommand::MoveSplit {
                                    node_id,
                                    child_idx: i,
                                    amount: total_splits_size * drag / size,
                                    min_size: total_splits_size * 30.0 / size
                                });
                            }
                            if response.hovered || response.dragging() {
                                ui.set_cursor(match direction {
                                    Axis::X => CursorIcon::EwResize,
                                    Axis::Y => CursorIcon::NsResize,
                                });
                            }
                        }
                    }
                });
            }
        }
        Some(())
    }

    fn render(&mut self, ui: &mut UI) {
        let mut commands = Vec::new();

        self.render_node(ui, self.root, &mut commands);

        for command in commands {
            self.execute_command(command);
        }
    }

}

impl<Tab: DockingTab> DockingState<Tab> {

    pub fn render(&mut self, ui: &mut UI) {
        ui.with_node(UINodeParams::new(Size::fr(1.0), Size::fr(1.0)), |ui| {
            self.tree.render(ui);
        });
    }

}

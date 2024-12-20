
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{vec2, Input, Memory, Painter, RawInput, Rect, RenderResources, UITree, Vec2, WindowConfig, UI};

use super::{Key, LogicalKey, TextRenderCache};

pub trait App {

    fn window_config() -> WindowConfig {
        WindowConfig::default()
    }

    fn tick(&mut self, ui: &mut UI);

}

struct AppHandler<'a, T: App> {
    app: T,
    render_resources: Option<RenderResources<'a>>,
    raw_input: RawInput,
    input: Input,
    memory: Memory
}

impl<T: App> AppHandler<'_, T> {

    pub fn tick(app: &mut T, render_resources: &mut RenderResources<'_>, raw_input: &mut RawInput, input: &mut Input, memory: &mut Memory) {
        let physical_size = vec2(render_resources.window.inner_size().width as f32, render_resources.window.inner_size().height as f32);
        let scale_factor = render_resources.window.scale_factor() as f32;
        let size = physical_size / scale_factor;
        
        let mut tree = UITree::new();
        let layer = tree.add_layer(size); 

        // distribute input
        input.update(raw_input, scale_factor); 
        input.distribute(memory);

        // ui generation
        let mut ui = UI::new(input, memory, render_resources, size, tree, layer);
        app.tick(&mut ui);
        let request_redraw = ui.request_redraw; 
        let mut tree = ui.tree();
        if request_redraw {
            render_resources.request_redraw();
        }

        memory.garbage_collect(&tree);

        // ui layout
        tree.layout(Rect::min_size(Vec2::ZERO, size), memory, &mut render_resources.text_resources);
        tree.remember_layout(memory);

        // ui rendering
        let Ok(output) = render_resources.surface.get_current_texture() else { return; }; 
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = render_resources.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("pierro_command_encoder"),
        });

        let mut next_text_render_cache = TextRenderCache::new();
        let mut painter = Painter::new(
            &render_resources.device,
            &render_resources.queue,
            &mut encoder,
            &view,
            
            &mut render_resources.paint_resources,
            &mut render_resources.text_resources, 

            size,
            scale_factor,

            &mut render_resources.text_render_cache,
            &mut next_text_render_cache
        );

        tree.paint(&mut painter);

        painter.finish();
        render_resources.text_render_cache = next_text_render_cache;

        render_resources.queue.submit([encoder.finish()]);
        output.present();

    }

}

fn winit_to_pierro_key(key: winit::keyboard::Key) -> Option<Key> {

    macro_rules! handle_logical_key {
        ($key: ident) => {
            if key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::$key) {
                return Some(Key {
                    text: None,
                    logical_key: Some(LogicalKey::$key)
                });
            }   
        };
    }

    handle_logical_key!(Alt);
    handle_logical_key!(CapsLock);
    handle_logical_key!(Control);
    handle_logical_key!(Fn);
    handle_logical_key!(Shift);
    if key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Super) {
        return Some(Key {
            text: None,
            logical_key: Some(LogicalKey::Command)
        });
    }
    handle_logical_key!(Enter);
    handle_logical_key!(Tab);
    handle_logical_key!(Space);
    handle_logical_key!(ArrowDown);
    handle_logical_key!(ArrowLeft);
    handle_logical_key!(ArrowRight);
    handle_logical_key!(ArrowUp);
    handle_logical_key!(Backspace);
    handle_logical_key!(Escape);
    handle_logical_key!(F1);
    handle_logical_key!(F2);
    handle_logical_key!(F3);
    handle_logical_key!(F4);
    handle_logical_key!(F5);
    handle_logical_key!(F6);
    handle_logical_key!(F7);
    handle_logical_key!(F8);
    handle_logical_key!(F9);
    handle_logical_key!(F10);
    handle_logical_key!(F11);
    handle_logical_key!(F12);

    if let winit::keyboard::Key::Character(text) = key {
        return Some(Key {
            text: Some(text.into()),
            logical_key: None,
        });
    }

    None
}

impl<T: App> ApplicationHandler for AppHandler<'_, T> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.render_resources.is_none() {
            self.render_resources = pollster::block_on(RenderResources::new(event_loop, T::window_config()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(render_resources) = &mut self.render_resources else { return; };
        if event != WindowEvent::RedrawRequested {
            render_resources.request_redraw();
        }
        match event {
            WindowEvent::Resized(new_size) => {
                render_resources.resize(new_size);
            },
            WindowEvent::RedrawRequested => {
                Self::tick(&mut self.app, render_resources, &mut self.raw_input, &mut self.input, &mut self.memory);
            },

            WindowEvent::MouseInput { device_id: _, state, button } => {
                match button {
                    MouseButton::Left => {
                        self.raw_input.l_mouse_down = state.is_pressed();
                    },
                    MouseButton::Right => {
                        self.raw_input.r_mouse_down = state.is_pressed();
                    },
                    _ => {}
                }
            },
            WindowEvent::CursorLeft { device_id: _ } => {
                self.raw_input.mouse_pos = None;
            },
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.raw_input.mouse_pos = Some(vec2(position.x as f32, position.y as f32))
            },
            WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.raw_input.scroll += vec2(x, y) * 5.0;
                    },
                    MouseScrollDelta::PixelDelta(physical_position) => {
                        self.raw_input.scroll += vec2(physical_position.x as f32, physical_position.y as f32);
                    },
                }
            },

            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if let Some(key) = winit_to_pierro_key(event.logical_key) {
                    if event.state.is_pressed() {
                        self.raw_input.keys_pressed.push(key);
                    } else {
                        self.raw_input.keys_released.push(key);
                    }
                }
            },

            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {} 
        }
    }

}

pub fn run<T: App>(app: T) {

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(&mut AppHandler {
        app,
        render_resources: None,
        raw_input: RawInput::new(),
        input: Input::new(),
        memory: Memory::new()
    }).unwrap();

}

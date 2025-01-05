use pierro::UINodeParams;


struct Gallery {
    docking_state: pierro::DockingState<GalleryTab>
}

struct Basic {
    text_edit_text: String,
    checkbox_state: bool
}

impl Basic {

    fn new() -> Self {
        Self {
            text_edit_text: "A text edit box".to_owned(),
            checkbox_state: true,
        }
    }

    fn ui(&mut self, ui: &mut pierro::UI) {
        pierro::scroll_area(ui, |ui| {
            pierro::label(ui, "A label");

            pierro::button(ui, "A button");

            pierro::text_edit(ui, &mut self.text_edit_text);

            pierro::checkbox_labeled(ui, "A checkbox", &mut self.checkbox_state);

            let context_menu_response = pierro::label(ui, "Context menu (right click me!)");
            pierro::context_menu(ui, &context_menu_response, |ui| {
                pierro::label(ui, "Inside the context menu");
            });

            pierro::collapsing_header(ui, "A collapsing header", |ui| {
                for i in 0..50 {
                    pierro::label(ui, i.to_string());
                }
            });
        });
    }

}

struct Layout {

}

impl Layout {
    
    fn new() -> Self {
        Self {

        }
    }

    fn node(&self, ui: &mut pierro::UI, color: pierro::Color) {
        ui.node(
            UINodeParams::new(pierro::Size::px(100.0), pierro::Size::px(100.0))
                .with_fill(color)
        );
    }

    fn ui(&mut self, ui: &mut pierro::UI) {
        pierro::container(ui,
            pierro::Size::fr(1.0),
            pierro::Size::fr(1.0),
            pierro::Layout::horizontal().align_center().justify_center(),
            |ui| {
                self.node(ui, pierro::Color::RED);   
                self.node(ui, pierro::Color::GREEN);   
                self.node(ui, pierro::Color::BLUE);
            });
    }

}

enum GalleryTab {
    Basic(Basic),
    Layout(Layout)
}

impl pierro::DockingTab for GalleryTab {

    fn title(&self) -> String {
        match self {
            GalleryTab::Basic(..) => "Basic Widgets".to_owned(),
            GalleryTab::Layout(..) => "Layout".to_owned()
        }
    }

    fn render(&mut self, ui: &mut pierro::UI) {
            match self {
                GalleryTab::Basic(basic) => basic.ui(ui),
                GalleryTab::Layout(layout) => layout.ui(ui)
            }
    }

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F) {
        if pierro::menu_button(ui, "Basic Widgets").mouse_released() {
            add_tab(Self::Basic(Basic::new()));
        }
        if pierro::menu_button(ui, "Layout").mouse_released() {
            add_tab(Self::Layout(Layout::new()));
        }
    }

}

impl pierro::App for Gallery {

    fn window_config() -> pierro::WindowConfig {
        pierro::WindowConfig::default()
            .with_title("Pierro Widget Gallery")
    }

    fn tick(&mut self, ui: &mut pierro::UI) {
        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "Menubar", |ui| {
                pierro::menu_button(ui, "Button A");
                pierro::menu_button(ui, "Button B");
                pierro::menu_button(ui, "Button C");
                pierro::menu_category(ui, "Category", |ui| {
                    pierro::menu_button(ui, "Button X");
                    pierro::menu_button(ui, "Button Y");
                });
            });
        });
        self.docking_state.render(ui);
    }

}

fn main() {
    pierro::run(Gallery {
        docking_state: pierro::DockingState::new(vec![
            GalleryTab::Basic(Basic::new())
        ])
    });
}
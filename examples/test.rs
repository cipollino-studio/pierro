
struct TestApp {
    n: i32,
    zoom: f32,
    text: String
}

impl pierro::App for TestApp {

    fn window_config() -> pierro::WindowConfig {
        pierro::WindowConfig::default()
            .with_title("Pierro Test")
    }

    fn tick(&mut self, ui: &mut pierro::UI) {

        let bg_color = ui.style::<pierro::Theme>().bg_dark;
        let bg = ui.node(
            pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fr(1.0))
                .with_layout(pierro::Layout::vertical())
                .with_fill(bg_color)
                .with_transform(pierro::TSTransform::scale(self.zoom))
        );
        
        ui.with_parent(bg.node_ref, |ui| {

            pierro::v_spacing(ui, 15.0);
            pierro::label(ui, "Above the scroll area");
            pierro::horizontal(ui, |ui| {
                if pierro::button(ui, "+").mouse_released() {
                    self.zoom *= 1.25;
                }
                if pierro::button(ui, "-").mouse_released() {
                    self.zoom /= 1.25;
                }
            });
            pierro::v_spacing(ui, 15.0);
            pierro::text_edit(ui, &mut self.text);
            pierro::v_spacing(ui, 15.0);

            pierro::scroll_area(ui, |ui| {

                pierro::label(ui, "Button #1: 🍊");
                if pierro::button(ui, "So long mom, I'm off to drop the bomb").mouse_released() {
                    self.n += 1;
                }

                pierro::label(ui, "Button #2:");
                if pierro::button(ui, "So long mom, I'm off to drop the bomb").mouse_released() {
                    self.n -= 1;
                }

                pierro::label(ui, "The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.");

                for i in 0..100 {
                    pierro::label(ui, format!("{} {}", i, self.n));
                }

            });

            pierro::v_spacing(ui, 15.0);
            pierro::label(ui, "Below the scroll area");
            pierro::label(ui, "Ниже области прокрутки");
            pierro::label(ui, "أسفل منطقة التمرير");
            pierro::v_spacing(ui, 15.0);

        });

        ui.layer(|ui| {
            pierro::v_spacing(ui, 150.0);
            if pierro::button(ui, "The button!!!").mouse_released() {
                self.n += 100;
            }
        });
        
    }

}

fn main() {
    pierro::run(TestApp {
        n: 0,
        zoom: 1.0,
        text: "Hello, World!".to_string() 
    });
}

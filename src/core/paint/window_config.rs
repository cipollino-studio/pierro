
use crate::{vec2, Vec2};

pub struct WindowConfig {
    pub(crate) title: String,
    pub(crate) min_size: Vec2
}

impl Default for WindowConfig {

    fn default() -> Self {
        Self {
            title: "Pierro Application".to_string(),
            min_size: vec2(400.0, 300.0)
        }
    }

}

impl WindowConfig {
    
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

}

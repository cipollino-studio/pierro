
pub struct WindowConfig {
    pub(crate) title: String
}

impl Default for WindowConfig {

    fn default() -> Self {
        Self {
            title: "Pierro Application".to_string()
        }
    }

}

impl WindowConfig {
    
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

}

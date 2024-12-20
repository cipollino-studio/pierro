
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum LogicalKey {
    Alt,
    CapsLock,
    Control,
    Fn,
    Shift,
    Command,

    Enter,
    Tab,
    Space,

    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    Backspace,
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Key {
    pub text: Option<String>,
    pub logical_key: Option<LogicalKey>
}

impl Key {

    pub const SHIFT: Self = Self {
        text: None,
        logical_key: Some(LogicalKey::Shift),
    };

}

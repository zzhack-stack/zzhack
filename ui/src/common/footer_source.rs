#[derive(PartialEq, Clone)]
pub struct Contact {
    pub link: &'static str,
    pub icon: &'static str,
    pub icon_size: i32,
}

pub const CONTACTS: [Contact; 0] = [];
pub const FOOTER_TEXT: &'static str = "";

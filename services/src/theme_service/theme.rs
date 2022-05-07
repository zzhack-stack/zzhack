#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn into_str(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn from(theme: &str) -> Theme {
        match theme {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => Theme::Light,
        }
    }

    pub fn eq(&self, other: &Theme) -> bool {
        self.clone().into_str() == other.clone().into_str()
    }

    pub fn nq(&self, other: &Theme) -> bool {
        !self.eq(other)
    }
}

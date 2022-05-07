#[macro_export]
macro_rules! use_style {
    ($arg:tt) => {{
        stylist::style!($arg).unwrap().get_class_name().to_string()
    }};
}

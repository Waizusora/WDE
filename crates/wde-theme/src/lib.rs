//! WDE's default monochrome visual tokens and window-control palette.

/// Crate identity for diagnostics and early integration tests.
pub const COMPONENT_NAME: &str = "wde-theme";

/// Supported built-in desktop themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinTheme {
    Black,
    White,
}

/// Hex RGB color token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// Semantic window-control colors. The controls are rendered as small circles
/// without glyphs; accessible names are supplied by the compositor UI layer.
pub const CLOSE_COLOR: Rgb = Rgb(0xEF, 0x44, 0x44);
pub const MAXIMIZE_RESTORE_COLOR: Rgb = Rgb(0xEA, 0xB3, 0x08);
pub const MINIMIZE_COLOR: Rgb = Rgb(0x22, 0xC5, 0x5E);

/// Window controls in visual order: close, maximize/restore, minimize.
pub const WINDOW_CONTROL_ORDER: [&str; 3] = ["close", "maximize_restore", "minimize"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_controls_use_the_wicofe_palette_and_order() {
        assert_eq!(CLOSE_COLOR, Rgb(239, 68, 68));
        assert_eq!(MAXIMIZE_RESTORE_COLOR, Rgb(234, 179, 8));
        assert_eq!(MINIMIZE_COLOR, Rgb(34, 197, 94));
        assert_eq!(
            WINDOW_CONTROL_ORDER,
            ["close", "maximize_restore", "minimize"]
        );
    }
}

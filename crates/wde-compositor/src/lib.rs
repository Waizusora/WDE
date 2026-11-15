//! Wayland compositor policy and window lifecycle primitives.
//!
//! The Smithay integration is optional. This crate owns WDE's window policy;
//! backend-specific protocol and rendering adapters are implemented separately.

/// Crate identity for diagnostics and early integration tests.
pub const COMPONENT_NAME: &str = "wde-compositor";

/// Logical state of a managed top-level window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Maximized,
    Minimized,
    Closed,
}

/// User action requested through WDE's window decoration or task list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAction {
    Close,
    MaximizeOrRestore,
    Minimize,
    Restore,
}

/// Result of applying an action to a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowActionError {
    AlreadyClosed,
}

/// Small policy state machine, independent of the Wayland backend.
///
/// A compositor adapter is responsible for translating these transitions into
/// xdg-shell configure events, focus changes, surface visibility, and task-list
/// updates. Minimized windows are not destroyed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowLifecycle {
    state: WindowState,
    state_before_minimize: WindowState,
}

impl Default for WindowLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowLifecycle {
    pub const fn new() -> Self {
        Self {
            state: WindowState::Normal,
            state_before_minimize: WindowState::Normal,
        }
    }

    pub const fn state(&self) -> WindowState {
        self.state
    }

    pub fn apply(&mut self, action: WindowAction) -> Result<WindowState, WindowActionError> {
        if self.state == WindowState::Closed {
            return Err(WindowActionError::AlreadyClosed);
        }

        match action {
            WindowAction::Close => self.state = WindowState::Closed,
            WindowAction::MaximizeOrRestore => {
                self.state = match self.state {
                    WindowState::Maximized => WindowState::Normal,
                    WindowState::Minimized => match self.state_before_minimize {
                        WindowState::Maximized => WindowState::Normal,
                        _ => WindowState::Maximized,
                    },
                    WindowState::Normal => WindowState::Maximized,
                    WindowState::Closed => unreachable!(),
                };
            }
            WindowAction::Minimize => {
                if self.state != WindowState::Minimized {
                    self.state_before_minimize = self.state;
                    self.state = WindowState::Minimized;
                }
            }
            WindowAction::Restore => {
                if self.state == WindowState::Minimized {
                    self.state = self.state_before_minimize;
                }
            }
        }

        Ok(self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximize_button_toggles_normal_and_maximized() {
        let mut window = WindowLifecycle::new();
        assert_eq!(
            window.apply(WindowAction::MaximizeOrRestore),
            Ok(WindowState::Maximized)
        );
        assert_eq!(
            window.apply(WindowAction::MaximizeOrRestore),
            Ok(WindowState::Normal)
        );
    }

    #[test]
    fn minimize_preserves_previous_state_until_restore() {
        let mut window = WindowLifecycle::new();
        window.apply(WindowAction::MaximizeOrRestore).unwrap();
        window.apply(WindowAction::Minimize).unwrap();
        assert_eq!(window.state(), WindowState::Minimized);
        window.apply(WindowAction::Restore).unwrap();
        assert_eq!(window.state(), WindowState::Maximized);
    }

    #[test]
    fn close_is_terminal() {
        let mut window = WindowLifecycle::new();
        window.apply(WindowAction::Close).unwrap();
        assert_eq!(
            window.apply(WindowAction::Restore),
            Err(WindowActionError::AlreadyClosed)
        );
    }
}

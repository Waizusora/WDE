//! Wayland compositor policy and window lifecycle primitives.
//!
//! Backend-independent window management lives here. Smithay adapters translate
//! these decisions into Wayland configure events, keyboard focus, visibility,
//! and panel notifications.

use std::collections::BTreeMap;

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

/// Stable compositor-local identifier assigned when a toplevel is registered.
pub type WindowId = u64;

/// Public, read-only snapshot for shell components such as the panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedWindow {
    pub id: WindowId,
    pub title: String,
    pub state: WindowState,
}

/// Errors returned by the backend-independent window registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowManagerError {
    UnknownWindow(WindowId),
    IdentifierExhausted,
}

/// Window registry and keyboard-focus policy.
///
/// The registry deliberately stores no Smithay or Wayland objects. A backend
/// owns those objects and uses these IDs to synchronize protocol events.
#[derive(Debug, Default)]
pub struct WindowManager {
    windows: BTreeMap<WindowId, ManagedWindow>,
    lifecycles: BTreeMap<WindowId, WindowLifecycle>,
    focused: Option<WindowId>,
    next_id: WindowId,
}

impl WindowManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn windows(&self) -> impl Iterator<Item = &ManagedWindow> {
        self.windows.values()
    }

    pub const fn focused(&self) -> Option<WindowId> {
        self.focused
    }

    pub fn get(&self, id: WindowId) -> Option<&ManagedWindow> {
        self.windows.get(&id)
    }

    /// Register a new toplevel and focus it. IDs are monotonic and never reused.
    pub fn register(&mut self, title: impl Into<String>) -> Result<WindowId, WindowManagerError> {
        let id = self.next_id;
        self.next_id = id
            .checked_add(1)
            .ok_or(WindowManagerError::IdentifierExhausted)?;

        self.windows.insert(
            id,
            ManagedWindow {
                id,
                title: title.into(),
                state: WindowState::Normal,
            },
        );
        self.lifecycles.insert(id, WindowLifecycle::new());
        self.focused = Some(id);
        Ok(id)
    }

    /// Focus an existing, non-minimized toplevel.
    pub fn focus(&mut self, id: WindowId) -> Result<(), WindowManagerError> {
        let window = self
            .windows
            .get(&id)
            .ok_or(WindowManagerError::UnknownWindow(id))?;
        if window.state != WindowState::Minimized {
            self.focused = Some(id);
        }
        Ok(())
    }

    /// Apply a window action and keep focus consistent with visibility.
    pub fn apply(
        &mut self,
        id: WindowId,
        action: WindowAction,
    ) -> Result<WindowState, WindowManagerError> {
        let lifecycle = self
            .lifecycles
            .get_mut(&id)
            .ok_or(WindowManagerError::UnknownWindow(id))?;
        let state = lifecycle
            .apply(action)
            .map_err(|_| WindowManagerError::UnknownWindow(id))?;

        if let Some(window) = self.windows.get_mut(&id) {
            window.state = state;
        }

        if self.focused == Some(id)
            && matches!(state, WindowState::Minimized | WindowState::Closed)
        {
            self.focused = self
                .windows
                .values()
                .rev()
                .find(|window| {
                    window.id != id
                        && window.state != WindowState::Minimized
                        && window.state != WindowState::Closed
                })
                .map(|window| window.id);
        } else if action == WindowAction::Restore {
            self.focused = Some(id);
        }

        if state == WindowState::Closed {
            self.windows.remove(&id);
            self.lifecycles.remove(&id);
        }
        Ok(state)
    }

    /// Remove a destroyed Wayland surface and choose a remaining visible focus target.
    pub fn unregister(&mut self, id: WindowId) -> Result<ManagedWindow, WindowManagerError> {
        let window = self
            .windows
            .remove(&id)
            .ok_or(WindowManagerError::UnknownWindow(id))?;
        self.lifecycles.remove(&id);

        if self.focused == Some(id) {
            self.focused = self
                .windows
                .values()
                .rev()
                .find(|candidate| candidate.state != WindowState::Minimized)
                .map(|candidate| candidate.id);
        }
        Ok(window)
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

    #[test]
    fn registering_windows_assigns_stable_ids_and_focuses_latest() {
        let mut manager = WindowManager::new();
        let first = manager.register("Terminal").unwrap();
        let second = manager.register("Settings").unwrap();

        assert_eq!((first, second), (0, 1));
        assert_eq!(manager.focused(), Some(second));
        assert_eq!(manager.get(first).unwrap().title, "Terminal");
    }

    #[test]
    fn minimizing_focused_window_moves_focus_to_visible_window() {
        let mut manager = WindowManager::new();
        let first = manager.register("Terminal").unwrap();
        let second = manager.register("Editor").unwrap();

        assert_eq!(
            manager.apply(second, WindowAction::Minimize),
            Ok(WindowState::Minimized)
        );
        assert_eq!(manager.focused(), Some(first));
        assert_eq!(manager.focus(second), Ok(()));
        assert_eq!(manager.focused(), Some(first));
    }

    #[test]
    fn closing_window_removes_it_and_selects_fallback_focus() {
        let mut manager = WindowManager::new();
        let first = manager.register("Terminal").unwrap();
        let second = manager.register("Editor").unwrap();

        assert_eq!(
            manager.apply(second, WindowAction::Close),
            Ok(WindowState::Closed)
        );
        assert!(manager.get(second).is_none());
        assert_eq!(manager.focused(), Some(first));
    }

    #[test]
    fn unregistering_unknown_window_is_reported() {
        let mut manager = WindowManager::new();
        assert_eq!(
            manager.unregister(42),
            Err(WindowManagerError::UnknownWindow(42))
        );
    }

    #[test]
    fn restoring_minimized_window_restores_state_and_focus() {
        let mut manager = WindowManager::new();
        let first = manager.register("Terminal").unwrap();
        let second = manager.register("Editor").unwrap();

        manager.apply(second, WindowAction::Minimize).unwrap();
        assert_eq!(manager.focused(), Some(first));
        assert_eq!(
            manager.apply(second, WindowAction::Restore),
            Ok(WindowState::Normal)
        );
        assert_eq!(manager.focused(), Some(second));
    }
}

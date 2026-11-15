//! WDE bottom-panel policy and task-list model.
//! Rendering and Wayland surface management belong to the future panel client.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelEdge {
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelVisibility {
    AlwaysVisible,
    AutoHide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Normal,
    Maximized,
    Minimized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelTask {
    pub window_id: u64,
    pub title: String,
    pub state: TaskState,
    pub focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelError {
    DuplicateWindow(u64),
    UnknownWindow(u64),
}

/// Backend-independent model for WDE's persistent bottom panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelModel {
    edge: PanelEdge,
    visibility: PanelVisibility,
    tasks: Vec<PanelTask>,
}

impl Default for PanelModel {
    fn default() -> Self {
        Self::new()
    }
}

impl PanelModel {
    pub fn new() -> Self {
        Self {
            edge: PanelEdge::Bottom,
            visibility: PanelVisibility::AlwaysVisible,
            tasks: Vec::new(),
        }
    }

    pub const fn edge(&self) -> PanelEdge {
        self.edge
    }

    pub const fn visibility(&self) -> PanelVisibility {
        self.visibility
    }

    pub fn tasks(&self) -> &[PanelTask] {
        &self.tasks
    }

    pub fn add_task(
        &mut self,
        window_id: u64,
        title: impl Into<String>,
    ) -> Result<(), PanelError> {
        if self.tasks.iter().any(|task| task.window_id == window_id) {
            return Err(PanelError::DuplicateWindow(window_id));
        }

        self.tasks.push(PanelTask {
            window_id,
            title: title.into(),
            state: TaskState::Normal,
            focused: false,
        });
        Ok(())
    }

    pub fn remove_task(&mut self, window_id: u64) -> Result<PanelTask, PanelError> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.window_id == window_id)
            .ok_or(PanelError::UnknownWindow(window_id))?;
        Ok(self.tasks.remove(index))
    }

    pub fn set_state(&mut self, window_id: u64, state: TaskState) -> Result<(), PanelError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.window_id == window_id)
            .ok_or(PanelError::UnknownWindow(window_id))?;
        task.state = state;
        Ok(())
    }

    pub fn focus(&mut self, window_id: Option<u64>) -> Result<(), PanelError> {
        if let Some(id) = window_id {
            if !self.tasks.iter().any(|task| task.window_id == id) {
                return Err(PanelError::UnknownWindow(id));
            }
        }

        for task in &mut self.tasks {
            task.focused = Some(task.window_id) == window_id;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_defaults_to_persistent_bottom_edge() {
        let panel = PanelModel::new();
        assert_eq!(panel.edge(), PanelEdge::Bottom);
        assert_eq!(panel.visibility(), PanelVisibility::AlwaysVisible);
    }

    #[test]
    fn task_lifecycle_and_focus_are_tracked() {
        let mut panel = PanelModel::new();
        panel.add_task(7, "Terminal").unwrap();
        panel.add_task(8, "Settings").unwrap();
        panel.focus(Some(8)).unwrap();
        panel.set_state(7, TaskState::Minimized).unwrap();

        assert!(!panel.tasks()[0].focused);
        assert_eq!(panel.tasks()[0].state, TaskState::Minimized);
        assert!(panel.tasks()[1].focused);

        panel.remove_task(7).unwrap();
        assert_eq!(panel.tasks().len(), 1);
    }

    #[test]
    fn duplicate_and_unknown_windows_are_rejected() {
        let mut panel = PanelModel::new();
        panel.add_task(1, "Terminal").unwrap();

        assert_eq!(
            panel.add_task(1, "Duplicate"),
            Err(PanelError::DuplicateWindow(1))
        );
        assert_eq!(panel.focus(Some(99)), Err(PanelError::UnknownWindow(99)));
    }
}

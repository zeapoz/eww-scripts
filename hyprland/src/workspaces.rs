use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use hyprland::{data::Monitors, event_listener::EventListener, shared::HyprData};
use serde::Serialize;
use serde_json::json;

use crate::Hyprland;

const WORKSPACE_COUNT: usize = 6;

#[derive(Debug, Serialize)]
pub struct Workspaces(Vec<Workspace>);

impl Workspaces {
    pub fn new() -> Self {
        let mut workspaces = Vec::with_capacity(WORKSPACE_COUNT);

        // Initialize with empty workspaces.
        for i in 1..=WORKSPACE_COUNT {
            let workspace = Workspace::new(i);
            workspaces.push(workspace)
        }

        // Update status of existing workspaces.
        for active_workspace in hyprland::data::Workspaces::get().unwrap() {
            let id = active_workspace.id as usize;
            let workspace = Workspace::from(active_workspace);
            workspaces.push(workspace);
            workspaces.swap_remove(id - 1);
        }

        Self(workspaces)
    }

    pub fn update(&mut self) {
        let mut workspaces = Vec::with_capacity(WORKSPACE_COUNT);

        // Initialize with empty workspaces.
        for i in 1..=WORKSPACE_COUNT {
            let workspace = Workspace::new(i);
            workspaces.push(workspace)
        }

        // Update status of existing workspaces.
        for active_workspace in hyprland::data::Workspaces::get().unwrap() {
            let id = active_workspace.id as usize;
            if let Some(workspace) = self.get_by_id(id) {
                let workspace = workspace.merge_with(active_workspace);
                workspaces.push(workspace);
                workspaces.swap_remove(id - 1);
            }
        }

        self.0 = workspaces
    }

    pub fn get_by_id(&mut self, id: usize) -> Option<&mut Workspace> {
        self.0.get_mut(id - 1)
    }
}

#[derive(Debug, Serialize)]
pub enum WorkspaceState {
    Empty,
    Active,
}

#[derive(Debug, Serialize)]
pub struct Workspace {
    pub id: usize,
    pub state: WorkspaceState,
    pub monitor: Option<i128>,
    pub urgent: bool,
}

impl Workspace {
    fn new(id: usize) -> Self {
        Self {
            id,
            state: WorkspaceState::Empty,
            monitor: None,
            urgent: false,
        }
    }

    fn merge_with(&self, other: hyprland::data::Workspace) -> Self {
        let id = other.id as usize;
        let monitor = *get_monitors().get(&other.monitor).unwrap();
        Self {
            id,
            state: WorkspaceState::Active,
            monitor: Some(monitor),
            urgent: self.urgent,
        }
    }
}

impl From<hyprland::data::Workspace> for Workspace {
    fn from(value: hyprland::data::Workspace) -> Self {
        let id = value.id as usize;
        let monitor = *get_monitors().get(&value.monitor).unwrap();
        Self {
            id,
            state: WorkspaceState::Active,
            monitor: Some(monitor),
            urgent: false,
        }
    }
}

pub fn add_workspace_handlers(hyprland: &Arc<Mutex<Hyprland>>, listener: &mut EventListener) {
    let hyprland_clone = hyprland.clone();
    listener.add_workspace_change_handler(move |id| {
        let mut hl = hyprland_clone.lock().unwrap();
        let id: usize = id.to_string().parse().unwrap();
        hl.focused = id;

        // Clear urgent status if workspace had one.
        if let Some(workspace) = hl.workspaces.get_by_id(id) {
            workspace.urgent = false;
        }

        println!("{}", json!(*hl));
    });

    let hyprland_clone = hyprland.clone();
    listener.add_active_monitor_change_handler(move |event| {
        let mut hl = hyprland_clone.lock().unwrap();
        let id: usize = event.workspace.to_string().parse().unwrap();
        hl.focused = id;

        // Clear urgent status if workspace had one.
        if let Some(workspace) = hl.workspaces.get_by_id(id) {
            workspace.urgent = false;
        }

        println!("{}", json!(*hl));
    });

    let hyprland_clone = hyprland.clone();
    let handle_add_remove = move |_| {
        hyprland_clone.lock().unwrap().workspaces.update();
        println!("{}", json!(*hyprland_clone));
    };
    listener.add_workspace_added_handler(handle_add_remove.clone());
    listener.add_workspace_destroy_handler(handle_add_remove.clone());
}

fn get_monitors() -> HashMap<String, i128> {
    Monitors::get()
        .map(|monitors| monitors.map(|m| (m.name, m.id)).collect::<HashMap<_, _>>())
        .unwrap()
}

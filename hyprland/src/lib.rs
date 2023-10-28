mod clients;
mod submap;
mod workspaces;

use std::sync::{Arc, Mutex};

use clients::Clients;
use hyprland::event_listener;
use serde::Serialize;
use serde_json::json;
use submap::Submap;
use workspaces::Workspaces;

use crate::{
    clients::add_clients_handler, submap::add_submap_handler, workspaces::add_workspace_handlers,
};

#[derive(Debug, Serialize)]
pub struct Hyprland {
    workspaces: Workspaces,
    clients: Clients,
    focused: usize,
    submap: Option<Submap>,
    urgent: String,
}

impl Hyprland {
    fn new() -> Self {
        Self {
            workspaces: Workspaces::new(),
            clients: Clients::update(),
            focused: 1,
            submap: None,
            urgent: String::new(),
        }
    }
}

#[tokio::main]
pub async fn subscribe() {
    let mut listener = event_listener::EventListener::new();

    let hyprland = Arc::new(Mutex::new(Hyprland::new()));
    println!("{}", json!(*hyprland));

    add_workspace_handlers(&hyprland, &mut listener);
    add_clients_handler(&hyprland, &mut listener);
    add_submap_handler(&hyprland, &mut listener);

    listener.start_listener_async().await.unwrap();
}

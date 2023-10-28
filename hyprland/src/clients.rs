use std::sync::{Arc, Mutex};

use hyprland::{
    event_listener::EventListener,
    shared::{Address, HyprData},
};
use serde::Serialize;
use serde_json::json;

use crate::Hyprland;

#[derive(Debug, Serialize)]
pub struct Clients(Vec<Client>);

impl Clients {
    pub fn update() -> Self {
        let mut clients = Vec::new();

        // Update status of existing workspaces.
        for client in hyprland::data::Clients::get().unwrap() {
            let client: Client = client.into();
            clients.push(client);
        }

        Self(clients)
    }

    pub fn find_by_address(&self, address: &str) -> Option<&Client> {
        self.0
            .iter()
            .find(|&client| client.address.to_string() == address)
    }
}

#[derive(Debug, Serialize)]
pub struct Client {
    pub address: Address,
    pub workspace: usize,
}

impl From<hyprland::data::Client> for Client {
    fn from(value: hyprland::data::Client) -> Self {
        Self {
            address: value.address,
            workspace: value.workspace.id as usize,
        }
    }
}

pub fn add_clients_handler(hyprland: &Arc<Mutex<Hyprland>>, listener: &mut EventListener) {
    let hyprland_clone = hyprland.clone();
    listener.add_window_moved_handler(move |_| {
        hyprland_clone.lock().unwrap().clients = Clients::update();
        println!("{}", json!(*hyprland_clone));
    });

    let hyprland_clone = hyprland.clone();
    listener.add_window_open_handler(move |_| {
        hyprland_clone.lock().unwrap().clients = Clients::update();
        println!("{}", json!(*hyprland_clone));
    });

    let hyprland_clone = hyprland.clone();
    listener.add_window_close_handler(move |_| {
        hyprland_clone.lock().unwrap().clients = Clients::update();
        println!("{}", json!(*hyprland_clone));
    });
}

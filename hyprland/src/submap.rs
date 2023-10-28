use std::sync::{Arc, Mutex};

use hyprland::event_listener::EventListener;
use serde::Serialize;
use serde_json::json;

use crate::Hyprland;

#[derive(Debug, Serialize)]
pub struct Submap {
    name: String,
    short: Option<String>,
}

impl Submap {
    fn new(name: String, short: Option<String>) -> Self {
        Self { name, short }
    }
}

impl From<String> for Submap {
    fn from(value: String) -> Self {
        match value.as_str() {
            "resize" => Submap::new(value, Some(String::from("rs"))),
            "group" => Submap::new(value, Some(String::from("gp"))),
            _ => Submap::new(value, None),
        }
    }
}

pub fn add_submap_handler(hyprland: &Arc<Mutex<Hyprland>>, listener: &mut EventListener) {
    let hyprland_clone = hyprland.clone();
    listener.add_sub_map_change_handler(move |map| {
        let map: Submap = map.into();
        let submap = match map.name.is_empty() {
            true => None,
            false => Some(map),
        };

        hyprland_clone.lock().unwrap().submap = submap;
        println!("{}", json!(*hyprland_clone));
    });
}

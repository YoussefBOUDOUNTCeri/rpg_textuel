#![allow(dead_code)]
use crate::utils::types_enums::PlaceType;

pub struct Lieu {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub place_type: PlaceType,
    pub connected_places: Vec<usize>,
    pub npcs_in_place: Vec<usize>,
    pub items_in_place: Vec<usize>,
}

impl Lieu {
    pub fn new(
        id: usize,
        name: &str,
        description: &str,
        place_type: PlaceType,
        connected_places: Vec<usize>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            place_type,
            connected_places,
            npcs_in_place: Vec::new(),
            items_in_place: Vec::new(),
        }
    }

    pub fn add_npc(&mut self, npc_id: usize) {
        if !self.npcs_in_place.contains(&npc_id) {
            self.npcs_in_place.push(npc_id);
        }
    }

    pub fn remove_npc(&mut self, npc_id: usize) {
        if let Some(index) = self.npcs_in_place.iter().position(|&id| id == npc_id) {
            self.npcs_in_place.remove(index);
        }
    }

    pub fn add_item(&mut self, item_id: usize) {
        if !self.items_in_place.contains(&item_id) {
            self.items_in_place.push(item_id);
        }
    }

    pub fn remove_item(&mut self, item_id: usize) {
        if let Some(index) = self.items_in_place.iter().position(|&id| id == item_id) {
            self.items_in_place.remove(index);
        }
    }
}
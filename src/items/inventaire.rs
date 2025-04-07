#![allow(dead_code)]
use crate::items::objet::Objet;

pub struct Inventaire {
    pub items: Vec<Objet>,
}

impl Inventaire {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Objet) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_id: usize) -> Option<Objet> {
        let index = self.items.iter().position(|x| x.id == item_id)?;
        Some(self.items.remove(index))
    }

    pub fn contains_item(&self, item_id: usize) -> bool {
        self.items.iter().any(|x| x.id == item_id)
    }
}
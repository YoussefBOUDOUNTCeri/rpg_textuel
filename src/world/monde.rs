#![allow(dead_code)]
use crate::world::lieu::Lieu;
use crate::world::gestionnaire_de_temps::GestionnaireDeTemps;
use crate::characters::pnj::PNJ;
use crate::events::evenement::Evenement;
use crate::items::objet::Objet;

pub struct Monde {
    pub places: Vec<Lieu>,
    pub npcs: Vec<PNJ>,
    pub time_manager: GestionnaireDeTemps,
    pub events: Vec<Evenement>,
}

impl Monde {
    pub fn new(
        places: Vec<Lieu>,
        npcs: Vec<PNJ>,
        time_manager: GestionnaireDeTemps,
        events: Vec<Evenement>,
    ) -> Self {
        Self {
            places,
            npcs,
            time_manager,
            events,
        }
    }

    pub fn update(&mut self) {
        self.time_manager.advance_time(1);
        self.resolve_events();
        for npc in self.npcs.iter_mut() {
            npc.do_ai();
        }
    }

    pub fn find_place_by_name(&self, name: &str) -> Option<&Lieu> {
        self.places.iter().find(|p| p.name == name)
    }

    pub fn find_place_by_id(&self, place_id: usize) -> Option<&Lieu> {
        self.places.iter().find(|p| p.id == place_id)
    }

    pub fn find_place_by_id_mut(&mut self, place_id: usize) -> Option<&mut Lieu> {
        self.places.iter_mut().find(|p| p.id == place_id)
    }

    pub fn schedule_event(&mut self, event: Evenement) {
        self.events.push(event);
    }

    pub fn resolve_events(&mut self) {
        let current_time = self.time_manager.current_time;
        let (due, future): (Vec<Evenement>, Vec<Evenement>) =
            self.events.drain(..).partition(|evt| evt.trigger_time <= current_time);
        for evt in due.iter() {
            evt.execute(self);
        }
        self.events = future;
    }

    pub fn get_item_by_id(&self, _item_id: usize) -> Option<Objet> {
        None
    }

    pub fn get_place_name(&self, place_id: usize) -> String {
        if let Some(place) = self.find_place_by_id(place_id) {
            place.name.clone()
        } else {
            "Lieu inconnu".to_string()
        }
    }
}
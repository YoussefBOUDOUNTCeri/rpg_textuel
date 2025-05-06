#![allow(dead_code)]
use crate::characters::personnage::{Personnage, Objet};
use crate::items::inventaire::Inventaire;
use crate::utils::types_enums::{Sex, CharacterState, NPCType, NPCState};
use crate::characters::joueur::Joueur;
use crate::utils::types_enums::ItemType;

pub struct PNJ {
    pub name: String,
    pub sex: Sex,
    pub age: u32,
    pub health: i32,
    pub hunger: i32,
    pub power: i32,
    pub aura: i32,
    pub money: i32,
    pub state: CharacterState,
    pub inventory: Inventaire,
    pub current_place: usize,
    pub npc_type: NPCType,
    pub ai_state: NPCState,
}

impl PNJ {
    pub fn do_ai(&mut self) {
        match self.ai_state {
            NPCState::Idle => {}
            NPCState::Patrol => {}
            NPCState::Attack => {}
            NPCState::Flee => {}
            NPCState::Follow => {}
        }
    }

    pub fn interact_with_player(&mut self, _player: &mut Joueur) {
    }
}

impl Personnage for PNJ {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn get_hunger(&self) -> i32 {
        self.hunger
    }

    fn get_money(&self) -> i32 {
        self.money
    }

    fn move_to(&mut self, place_id: usize) {
        self.current_place = place_id;
    }

    fn eat(&mut self, item: &Objet) {
        if let Some(effect) = &item.effects {
            match effect {
                crate::characters::personnage::Effect::Heal(v) => {
                    self.health += v;
                    if self.health > 100 {
                        self.health = 100;
                    }
                }
                crate::characters::personnage::Effect::Damage(v) => {
                    self.health -= v;
                    if self.health < 0 {
                        self.health = 0;
                    }
                }
                crate::characters::personnage::Effect::Buff => {}
                crate::characters::personnage::Effect::Debuff => {}
            }
        }
        if item.item_type == ItemType::Food {
            if self.hunger > 0 {
                self.hunger -= 5;
                if self.hunger < 0 {
                    self.hunger = 0;
                }
            }
        }
    }

    fn fight(&mut self, target: &mut dyn Personnage) {
        if self.power > 0 {
            target.receive_damage(self.power);
        }
    }

    fn talk(&self, _target: &dyn Personnage) {
    }

    fn work(&mut self) {
        self.money += 5;
    }

    fn set_health(&mut self, value: i32) {
        self.health = value;
    }

    fn set_hunger(&mut self, value: i32) {
        self.hunger = value;
    }

    fn receive_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
}
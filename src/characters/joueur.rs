#![allow(dead_code)]
use crate::characters::personnage::{Personnage, Objet};
use crate::utils::types_enums::ItemType;
use crate::items::inventaire::Inventaire;
use crate::progression::progression::Progression;
use crate::utils::types_enums::{Sex, CharacterState, PathType};

pub struct Joueur {
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
    pub experience: i32,
    pub level: i32,
    pub current_progress: Progression,
    pub bank_balance: i32,
}

impl Joueur {
    pub fn add_experience(&mut self, amount: i32) {
        self.experience += amount;
        if self.experience >= 100 {
            self.experience = 0;
            self.level += 1;
        }
    }

    pub fn choose_path(&mut self, path: PathType) {
        self.current_progress.path_type = path;
    }

    pub fn deposit_money(&mut self, amount: i32) {
        if amount > 0 && self.money >= amount {
            self.money -= amount;
            self.bank_balance += amount;
        }
    }

    pub fn withdraw_money(&mut self, amount: i32) {
        if amount > 0 && self.bank_balance >= amount {
            self.bank_balance -= amount;
            self.money += amount;
        }
    }
}

impl Personnage for Joueur {
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
                self.hunger -= 10;
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
        self.money += 10;
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

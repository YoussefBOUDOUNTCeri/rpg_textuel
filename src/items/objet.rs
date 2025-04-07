#![allow(dead_code)]
use crate::utils::types_enums::ItemType;
use crate::characters::personnage::{Personnage, Effect};

#[derive(Clone)]
pub struct Objet {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub value: i32,
    pub effects: Option<Effect>,
}

impl Objet {
    pub fn use_item(&self, target: &mut dyn Personnage) {
        if let Some(effect) = &self.effects {
            match effect {
                Effect::Heal(v) => {
                    let result = target.get_health() + v;
                    if result > 100 {
                        target.set_health(100);
                    } else {
                        target.set_health(result);
                    }
                }
                Effect::Damage(v) => {
                    target.receive_damage(*v);
                }
                Effect::Buff => {}
                Effect::Debuff => {}
            }
        }
        if self.item_type == ItemType::Food {
            let hunger = target.get_hunger();
            let updated = if hunger < 10 { 0 } else { hunger - 10 };
            target.set_hunger(updated);
        }
    }
}

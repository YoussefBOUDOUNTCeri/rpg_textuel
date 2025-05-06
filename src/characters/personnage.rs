
pub use crate::utils::types_enums::ItemType;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Heal(i32),
    Damage(i32),
    Buff,
    Debuff,
}

pub struct Objet {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub value: i32,
    pub effects: Option<Effect>,
}

pub trait Personnage {
    fn get_name(&self) -> &str;
    fn get_health(&self) -> i32;
    fn get_hunger(&self) -> i32;
    fn get_money(&self) -> i32;
    fn move_to(&mut self, place_id: usize);
    fn eat(&mut self, item: &Objet);
    fn fight(&mut self, target: &mut dyn Personnage);
    fn talk(&self, target: &dyn Personnage);
    fn work(&mut self);

    fn set_health(&mut self, value: i32);
    fn set_hunger(&mut self, value: i32);

    fn receive_damage(&mut self, amount: i32);
}
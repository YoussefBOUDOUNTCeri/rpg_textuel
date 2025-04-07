#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceType {
    Safe,
    Dangerous,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPCType {
    Banker,
    Policeman,
    Dealer,
    GangMember,
    Civilian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPCState {
    Idle,
    Patrol,
    Attack,
    Flee,
    Follow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterState {
    Normal,
    Wounded,
    InPrison,
    Dead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Food,
    Weapon,
    Drug,
    Money,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Walk,
    Eat,
    Fight,
    Talk,
    Work,
    Pickup,
    UseItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    RandomEncounter,
    PoliceRaid,
    ScheduledMeeting,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathType {
    Legal,
    Illegal,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
    Other,
}
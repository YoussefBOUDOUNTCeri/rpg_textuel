// tests/utils_test.rs

use rpg_textuel::utils::types_enums::{
    PlaceType, NPCType, NPCState, CharacterState, ItemType, ActionType, EventType, PathType, Sex,
};

#[test]
fn test_place_type_enum() {
    let safe = PlaceType::Safe;
    let dangerous = PlaceType::Dangerous;
    let neutral = PlaceType::Neutral;

    assert_eq!(safe, PlaceType::Safe);
    assert_eq!(dangerous, PlaceType::Dangerous);
    assert_eq!(neutral, PlaceType::Neutral);
}

#[test]
fn test_npc_type_enum() {
    let banker = NPCType::Banker;
    let dealer = NPCType::Dealer;

    assert_eq!(banker, NPCType::Banker);
    assert_eq!(dealer, NPCType::Dealer);
}

#[test]
fn test_npc_state_enum() {
    let idle = NPCState::Idle;
    let attack = NPCState::Attack;

    assert_eq!(idle, NPCState::Idle);
    assert_eq!(attack, NPCState::Attack);
}

#[test]
fn test_character_state_enum() {
    let normal = CharacterState::Normal;
    let dead = CharacterState::Dead;

    assert_eq!(normal, CharacterState::Normal);
    assert_eq!(dead, CharacterState::Dead);
}

#[test]
fn test_item_type_enum() {
    let food = ItemType::Food;
    let weapon = ItemType::Weapon;

    assert_eq!(food, ItemType::Food);
    assert_eq!(weapon, ItemType::Weapon);
}

#[test]
fn test_action_type_enum() {
    let walk = ActionType::Walk;
    let fight = ActionType::Fight;

    assert_eq!(walk, ActionType::Walk);
    assert_eq!(fight, ActionType::Fight);
}

#[test]
fn test_event_type_enum() {
    let random_encounter = EventType::RandomEncounter;
    let police_raid = EventType::PoliceRaid;

    assert_eq!(random_encounter, EventType::RandomEncounter);
    assert_eq!(police_raid, EventType::PoliceRaid);
}

#[test]
fn test_path_type_enum() {
    let legal = PathType::Legal;
    let illegal = PathType::Illegal;
    let neutral = PathType::Neutral;

    assert_eq!(legal, PathType::Legal);
    assert_eq!(illegal, PathType::Illegal);
    assert_eq!(neutral, PathType::Neutral);
}

#[test]
fn test_sex_enum() {
    let male = Sex::Male;
    let female = Sex::Female;
    let other = Sex::Other;

    assert_eq!(male, Sex::Male);
    assert_eq!(female, Sex::Female);
    assert_eq!(other, Sex::Other);
}

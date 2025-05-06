use rpg_textuel::characters::joueur::Joueur;
use rpg_textuel::characters::pnj::PNJ;
use rpg_textuel::characters::personnage::Personnage;
use rpg_textuel::items::inventaire::Inventaire;
use rpg_textuel::progression::progression::Progression;
use rpg_textuel::utils::types_enums::{Sex, CharacterState, PathType, NPCType, NPCState};

#[test]
fn test_joueur_creation() {
    let inventaire = Inventaire::new();
    let progression = Progression::new(PathType::Legal, 0, 10);
    let joueur = Joueur {
        name: String::from("Testeur"),
        sex: Sex::Male,
        age: 20,
        health: 100,
        hunger: 50,
        power: 20,
        aura: 15,
        money: 200,
        state: CharacterState::Normal,
        inventory: inventaire,
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: progression,
        bank_balance: 0,
    };

    assert_eq!(joueur.get_name(), "Testeur");
    assert_eq!(joueur.get_health(), 100);
    assert_eq!(joueur.get_money(), 200);
}

#[test]
fn test_joueur_experience_et_level_up() {
    let mut joueur = Joueur {
        name: String::from("XPMan"),
        sex: Sex::Male,
        age: 25,
        health: 100,
        hunger: 50,
        power: 20,
        aura: 15,
        money: 100,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 95,
        level: 1,
        current_progress: Progression::new(PathType::Neutral, 0, 10),
        bank_balance: 0,
    };

    joueur.add_experience(10);
    assert_eq!(joueur.experience, 0);
    assert_eq!(joueur.level, 2);
}

#[test]
fn test_joueur_banque_operations() {
    let mut joueur = Joueur {
        name: String::from("BankMan"),
        sex: Sex::Male,
        age: 30,
        health: 100,
        hunger: 50,
        power: 20,
        aura: 10,
        money: 100,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Legal, 0, 10),
        bank_balance: 0,
    };

    joueur.deposit_money(50);
    assert_eq!(joueur.money, 50);
    assert_eq!(joueur.bank_balance, 50);

    joueur.withdraw_money(30);
    assert_eq!(joueur.money, 80);
    assert_eq!(joueur.bank_balance, 20);
}

#[test]
fn test_joueur_combat() {
    let mut joueur = Joueur {
        name: String::from("Fighter"),
        sex: Sex::Male,
        age: 20,
        health: 100,
        hunger: 50,
        power: 20,
        aura: 10,
        money: 100,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Neutral, 0, 10),
        bank_balance: 0,
    };

    let mut adversaire = Joueur {
        name: String::from("Victim"),
        sex: Sex::Female,
        age: 18,
        health: 100,
        hunger: 50,
        power: 10,
        aura: 5,
        money: 50,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Neutral, 0, 10),
        bank_balance: 0,
    };

    joueur.fight(&mut adversaire);
    assert_eq!(adversaire.get_health(), 80);
}

#[test]
fn test_pnj_basic_behavior() {
    let mut pnj = PNJ {
        name: "Bob le vendeur".to_string(),
        sex: Sex::Male,
        age: 40,
        health: 100,
        hunger: 30,
        power: 10,
        aura: 5,
        money: 20,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        npc_type: NPCType::Banker,
        ai_state: NPCState::Idle,
    };

    assert_eq!(pnj.get_name(), "Bob le vendeur");
    assert_eq!(pnj.get_health(), 100);

    // Test déplacement
    pnj.move_to(1);
    assert_eq!(pnj.current_place, 1);
}

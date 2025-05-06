use rpg_textuel::scenario::scenarios::{ScenarioManager, Scenario};
use rpg_textuel::characters::joueur::Joueur;
use rpg_textuel::items::inventaire::Inventaire;
use rpg_textuel::progression::progression::Progression;
use rpg_textuel::utils::types_enums::{Sex, CharacterState, PathType};
use rpg_textuel::characters::personnage::Personnage;


#[test]
fn test_scenario_manager_load() {
    let manager = ScenarioManager::load_from_file("scenarios/city.xml");
    assert!(!manager.scenarios.is_empty());
}

#[test]
fn test_get_first_scenario() {
    let manager = ScenarioManager::load_from_file("scenarios/city.xml");
    let scenario = manager.get_current_scenario();
    assert!(scenario.is_some());
}

#[test]
fn test_set_and_get_specific_scenario() {
    let mut manager = ScenarioManager::load_from_file("scenarios/city.xml");

    manager.set_current_scenario("aller_boulangerie".to_string());
    let scenario = manager.get_current_scenario();

    assert!(scenario.is_some());
    assert_eq!(scenario.unwrap().id, "aller_boulangerie");
}

#[test]
fn test_apply_effects_health() {
    let mut joueur = create_test_joueur();
    let scenario = Scenario {
        id: "test_effect".to_string(),
        description: "Test scénario avec effet de santé.".to_string(),
        effects: vec!["health + 10".to_string()],
        choices: vec![],
    };

    let manager = ScenarioManager {
        scenarios: vec![scenario.clone()],
        current_id: Some(scenario.id.clone()),
    };

    joueur.set_health(50);
    manager.apply_effects(&mut joueur);
    assert_eq!(joueur.get_health(), 60);
}

#[test]
fn test_apply_effects_money() {
    let mut joueur = create_test_joueur();
    let scenario = Scenario {
        id: "test_money".to_string(),
        description: "Test scénario avec effet d'argent.".to_string(),
        effects: vec!["money + 100".to_string()],
        choices: vec![],
    };

    let manager = ScenarioManager {
        scenarios: vec![scenario.clone()],
        current_id: Some(scenario.id.clone()),
    };

    joueur.money = 50;
    manager.apply_effects(&mut joueur);
    assert_eq!(joueur.money, 150);
}

// ========================================
// Fonction utilitaire pour initialiser un Joueur de test
// ========================================
fn create_test_joueur() -> Joueur {
    Joueur {
        name: "Testeur".to_string(),
        sex: Sex::Male,
        age: 25,
        health: 100,
        hunger: 50,
        power: 20,
        aura: 15,
        money: 200,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Neutral, 0, 10),
        bank_balance: 0,
    }
}

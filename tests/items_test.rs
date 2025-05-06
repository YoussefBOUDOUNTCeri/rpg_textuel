use rpg_textuel::items::inventaire::Inventaire;
use rpg_textuel::items::objet::Objet;
use rpg_textuel::characters::personnage::{Effect, ItemType, Personnage};
use rpg_textuel::characters::joueur::Joueur;
use rpg_textuel::progression::progression::Progression;
use rpg_textuel::utils::types_enums::{PathType, Sex, CharacterState};
    
#[test]
fn test_inventaire_add_and_contains() {
    let mut inventaire = Inventaire::new();
    let objet = Objet {
        id: 1,
        name: "Potion de soin".to_string(),
        description: "Restaure la santé.".to_string(),
        item_type: ItemType::Food,
        value: 10,
        effects: Some(Effect::Heal(20)),
    };

    inventaire.add_item(objet.clone());
    assert!(inventaire.contains_item(1));
}

#[test]
fn test_inventaire_remove() {
    let mut inventaire = Inventaire::new();
    let objet = Objet {
        id: 2,
        name: "Épée rouillée".to_string(),
        description: "Pas très efficace, mais mieux que rien.".to_string(),
        item_type: ItemType::Weapon,
        value: 50,
        effects: None,
    };

    inventaire.add_item(objet.clone());
    let removed = inventaire.remove_item(2);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name, "Épée rouillée");
    assert!(!inventaire.contains_item(2));
}

#[test]
fn test_objet_use_item_heal() {
    let mut joueur = create_test_joueur();
    let objet = Objet {
        id: 3,
        name: "Potion de soin majeure".to_string(),
        description: "Restaure beaucoup de santé.".to_string(),
        item_type: ItemType::Food,
        value: 25,
        effects: Some(Effect::Heal(50)),
    };

    joueur.set_health(40);
    objet.use_item(&mut joueur);
    assert_eq!(joueur.get_health(), 90);
}

#[test]
fn test_objet_use_item_damage() {
    let mut joueur = create_test_joueur();
    let objet = Objet {
        id: 4,
        name: "Potion empoisonnée".to_string(),
        description: "Fait perdre de la santé.".to_string(),
        item_type: ItemType::Food,
        value: 15,
        effects: Some(Effect::Damage(30)),
    };

    joueur.set_health(100);
    objet.use_item(&mut joueur);
    assert_eq!(joueur.get_health(), 70);
}

#[test]
fn test_objet_use_item_hunger() {
    let mut joueur = create_test_joueur();
    let objet = Objet {
        id: 5,
        name: "Sandwich".to_string(),
        description: "Nourrit et restaure un peu de santé.".to_string(),
        item_type: ItemType::Food,
        value: 5,
        effects: None,
    };

    joueur.set_hunger(20);
    objet.use_item(&mut joueur);
    assert_eq!(joueur.get_hunger(), 10);
}

// ====================
// Fonctions utilitaires pour les tests
// ====================
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

use rpg_textuel::game_engine::moteur_de_jeu::{MoteurDeJeu, MenuPrincipal};
use rpg_textuel::characters::joueur::Joueur;
use rpg_textuel::world::monde::Monde;
use rpg_textuel::world::lieu::Lieu;
use rpg_textuel::world::gestionnaire_de_temps::GestionnaireDeTemps;
use rpg_textuel::progression::progression::Progression;
use rpg_textuel::utils::types_enums::{PlaceType, Sex, CharacterState, PathType};
use rpg_textuel::items::inventaire::Inventaire;

#[test]
fn test_moteur_de_jeu_initialisation() {
    let lieu = Lieu::new(0, "Maison", "Votre maison", PlaceType::Safe, vec![]);
    let time_manager = GestionnaireDeTemps::new(0, 1440);
    let monde = Monde::new(vec![lieu], vec![], time_manager, vec![]);

    let joueur = Joueur {
        name: "Testeur".to_string(),
        sex: Sex::Male,
        age: 25,
        health: 100,
        hunger: 50,
        power: 30,
        aura: 10,
        money: 200,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Neutral, 0, 10),
        bank_balance: 0,
    };

    let moteur = MoteurDeJeu::new(monde, joueur);

    assert!(moteur.is_running);
    assert_eq!(moteur.current_menu, MenuPrincipal::Accueil);
    assert!(moteur.history.is_empty());
}

#[test]
fn test_moteur_de_jeu_arret() {
    let monde = Monde::new(vec![], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);
    let joueur = Joueur {
        name: "Testeur".to_string(),
        sex: Sex::Male,
        age: 20,
        health: 100,
        hunger: 0,
        power: 10,
        aura: 5,
        money: 50,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 0,
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Legal, 0, 10),
        bank_balance: 0,
    };

    let mut moteur = MoteurDeJeu::new(monde, joueur);
    moteur.stop();

    assert!(!moteur.is_running);
}

#[test]
fn test_moteur_de_jeu_deplacement() {
    let lieu_depart = Lieu::new(0, "Maison", "Départ", PlaceType::Safe, vec![1]);
    let lieu_arrivee = Lieu::new(1, "Ville", "Destination", PlaceType::Neutral, vec![0]);
    let time_manager = GestionnaireDeTemps::new(0, 1440);
    let monde = Monde::new(vec![lieu_depart, lieu_arrivee], vec![], time_manager, vec![]);

    let joueur = Joueur {
        name: "Testeur".to_string(),
        sex: Sex::Male,
        age: 20,
        health: 100,
        hunger: 0,
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

    let mut moteur = MoteurDeJeu::new(monde, joueur);

    // Simuler qu'on a affiché les lieux connectés
    moteur.current_connected_places = vec![1];

    // Simuler la sélection du déplacement vers la Ville
    moteur.gerer_menu_deplacement("1".to_string());

    assert_eq!(moteur.player.current_place, 1);
    assert_eq!(moteur.current_menu, MenuPrincipal::Accueil);
}

#[test]
fn test_moteur_de_jeu_mise_a_jour_et_mort() {
    let monde = Monde::new(vec![], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);

    let joueur = Joueur {
        name: "Testeur".to_string(),
        sex: Sex::Male,
        age: 20,
        health: 0, // déjà mort
        hunger: 0,
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

    let mut moteur = MoteurDeJeu::new(monde, joueur);

    moteur.mettre_a_jour();

    assert!(!moteur.is_running);
}

#[test]
fn test_get_scenario_file() {
    let monde = Monde::new(vec![], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);

    let joueur = Joueur {
        name: "Testeur".to_string(),
        sex: Sex::Male,
        age: 20,
        health: 100,
        hunger: 0,
        power: 10,
        aura: 5,
        money: 50,
        state: CharacterState::Normal,
        inventory: Inventaire::new(),
        current_place: 2, // Banque
        experience: 0,
        level: 1,
        current_progress: Progression::new(PathType::Neutral, 0, 10),
        bank_balance: 0,
    };

    let moteur = MoteurDeJeu::new(monde, joueur);

    let path = moteur.get_scenario_file();
    assert_eq!(path, "scenarios/bank.xml");
}

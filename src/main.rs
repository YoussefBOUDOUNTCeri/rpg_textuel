mod game_engine;
mod world;
mod characters;
mod items;
mod events;
mod progression;
mod dialogue;
mod utils;
pub mod scenario;


use crate::game_engine::moteur_de_jeu::MoteurDeJeu;
use crate::characters::joueur::Joueur;
use crate::world::monde::Monde;
use crate::world::gestionnaire_de_temps::GestionnaireDeTemps;
use crate::world::lieu::Lieu;
use crate::items::inventaire::Inventaire;
use crate::progression::progression::Progression;
use crate::utils::types_enums::{
    PlaceType, PathType, Sex, EventType, CharacterState
};
use crate::events::evenement::{Evenement, EventData};
use rpg_textuel::{prompt, prompt_string};


fn main() {
    let name = prompt_string("Entrez le nom de votre personnage [Jean] : ", "Jean");
    let age: u32 = prompt("Entrez l'âge de votre personnage [18] : ", 18);
    
    // Sélection du sexe
    println!("Choisissez votre sexe :");
    println!("1) Homme");
    println!("2) Femme");
    println!("3) Autre");
    let sexe_choice: u32 = prompt("Votre choix [1] : ", 1);
    let sex = match sexe_choice {
        1 => Sex::Male,
        2 => Sex::Female,
        3 => Sex::Other,
        _ => Sex::Male, // Par défaut Homme
    };
    
    let health: i32 = 75;
    let money: i32 = 150;

    let lieu0 = Lieu::new(
        0,
        "Home",
        "Une zone calme avec des pavillons.",
        PlaceType::Safe,
        vec![1, 4, 5, 7],
    );
    let lieu1 = Lieu::new(
        1,
        "Centre-Ville",
        "Un grand centre urbain, très animé.",
        PlaceType::Neutral,
        vec![0, 2, 3, 6, 8],
    );
    let lieu2 = Lieu::new(
        2,
        "Banque",
        "Un établissement financier sécurisé.",
        PlaceType::Safe,
        vec![1],
    );
    let lieu3 = Lieu::new(
        3,
        "Supermarché",
        "Un grand magasin pour faire des courses.",
        PlaceType::Neutral,
        vec![1],
    );
    let lieu4 = Lieu::new(
        4,
        "Banlieue mal famée",
        "Un endroit dangereux où rôdent des gangs.",
        PlaceType::Dangerous,
        vec![0],
    );
    let lieu5 = Lieu::new(
        5,
        "Plage",
        "Nager.",
        PlaceType::Neutral,
        vec![1],
    );
    let lieu6 = Lieu::new(
        6,
        "Hopital",
        "Un endroit pour se soigner.",
        PlaceType::Safe,
        vec![1],
    );
    let lieu7 = Lieu::new(
        7,
        "Prison",
        "La prison c'est dur, la sortie c'est sur",
        PlaceType::Dangerous,
        vec![0],
    );
    let lieu8 = Lieu::new(
        8,
        "Basic Fit",
        "La prison c'est dur, la sortie c'est sur",
        PlaceType::Neutral,
        vec![1],
    );
    let places = vec![lieu0, lieu1, lieu2, lieu3, lieu4, lieu5, lieu6, lieu7, lieu8];

    let time_manager = GestionnaireDeTemps::new(0, 1440);

    let events = vec![
        Evenement::new(
            0,
            10,
            EventType::RandomEncounter,
            Some(EventData {
                description: "Une rencontre aléatoire survient.".to_string(),
                _target_id: None,
            }),
        ),
        Evenement::new(
            1,
            20,
            EventType::PoliceRaid,
            Some(EventData {
                description: "La police débarque pour un raid surprise.".to_string(),
                _target_id: None,
            }),
        ),
    ];
    let pnj_list = vec![];

    let world = Monde::new(places, pnj_list, time_manager, events);

    let player_inventory = Inventaire::new();
    let player_progression = Progression::new(PathType::Legal, 0, 10);

    let player = Joueur {
        name,
        sex,
        age,
        health,
        hunger: 50,
        power: 50,
        aura: 10,
        money,
        state: CharacterState::Normal,
        inventory: player_inventory,
        current_place: 0,
        experience: 1,
        level: 1,
        current_progress: player_progression,
        bank_balance: 100,
    };

    let mut moteur = MoteurDeJeu::new(world, player);
    moteur.run();
}

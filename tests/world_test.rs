use rpg_textuel::world::monde::Monde;
use rpg_textuel::world::lieu::Lieu;
use rpg_textuel::world::gestionnaire_de_temps::GestionnaireDeTemps;
use rpg_textuel::utils::types_enums::PlaceType;
use rpg_textuel::events::evenement::{Evenement, EventData};
use rpg_textuel::utils::types_enums::EventType;

#[test]
fn test_creation_lieu() {
    let lieu = Lieu::new(
        0,
        "Maison",
        "Une maison confortable",
        PlaceType::Safe,
        vec![1, 2],
    );

    assert_eq!(lieu.id, 0);
    assert_eq!(lieu.name, "Maison");
    assert_eq!(lieu.place_type, PlaceType::Safe);
    assert_eq!(lieu.connected_places, vec![1, 2]);
}

#[test]
fn test_gestion_npc_dans_lieu() {
    let mut lieu = Lieu::new(0, "Ville", "Ville animée", PlaceType::Neutral, vec![]);

    lieu.add_npc(5);
    assert!(lieu.npcs_in_place.contains(&5));

    lieu.remove_npc(5);
    assert!(!lieu.npcs_in_place.contains(&5));
}

#[test]
fn test_gestion_item_dans_lieu() {
    let mut lieu = Lieu::new(1, "Supermarché", "Un grand supermarché", PlaceType::Neutral, vec![]);

    lieu.add_item(10);
    assert!(lieu.items_in_place.contains(&10));

    lieu.remove_item(10);
    assert!(!lieu.items_in_place.contains(&10));
}

#[test]
fn test_temps_avance_dans_monde() {
    let lieu = Lieu::new(0, "Maison", "Endroit calme", PlaceType::Safe, vec![]);
    let mut monde = Monde::new(vec![lieu], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);

    assert_eq!(monde.time_manager.current_time, 0);
    monde.update();
    assert_eq!(monde.time_manager.current_time, 1);
}

#[test]
fn test_find_place_by_name_et_id() {
    let lieu1 = Lieu::new(0, "Maison", "Endroit calme", PlaceType::Safe, vec![]);
    let lieu2 = Lieu::new(1, "Ville", "Centre ville", PlaceType::Neutral, vec![]);
    let monde = Monde::new(vec![lieu1, lieu2], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);

    let found = monde.find_place_by_name("Ville");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, 1);

    let found_by_id = monde.find_place_by_id(0);
    assert!(found_by_id.is_some());
    assert_eq!(found_by_id.unwrap().name, "Maison");
}

#[test]
fn test_schedule_and_resolve_events() {
    let lieu = Lieu::new(0, "Maison", "Endroit calme", PlaceType::Safe, vec![]);
    let mut monde = Monde::new(vec![lieu], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);

    let event = Evenement::new(
        1,
        0,
        EventType::RandomEncounter,
        Some(EventData {
            description: "Rencontre surprise".to_string(),
            _target_id: None,
        }),
    );

    monde.schedule_event(event);
    assert_eq!(monde.events.len(), 1);

    monde.resolve_events();
    assert_eq!(monde.events.len(), 0);
}

#[test]
fn test_get_place_name() {
    let lieu = Lieu::new(0, "Maison", "Votre maison", PlaceType::Safe, vec![]);
    let monde = Monde::new(vec![lieu], vec![], GestionnaireDeTemps::new(0, 1440), vec![]);

    let name = monde.get_place_name(0);
    assert_eq!(name, "Maison");

    let unknown = monde.get_place_name(99);
    assert_eq!(unknown, "Lieu inconnu");
}

#[test]
fn test_gestionnaire_de_temps_fonctions() {
    let mut temps = GestionnaireDeTemps::new(0, 1440);

    temps.advance_time(60);
    assert_eq!(temps.current_time, 60);

    assert!(temps.is_daytime());

    let jour = temps.get_current_day();
    assert_eq!(jour, 0);

    let heure = temps.get_current_hour();
    assert_eq!(heure, 1);

    let minute = temps.get_current_minute();
    assert_eq!(minute, 0);
}
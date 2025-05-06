use rpg_textuel::events::evenement::{Evenement, EventData};
use rpg_textuel::utils::types_enums::EventType;
use rpg_textuel::world::gestionnaire_de_temps::GestionnaireDeTemps;
use rpg_textuel::world::lieu::Lieu;
use rpg_textuel::world::monde::Monde;
use rpg_textuel::utils::types_enums::PlaceType;

#[test]
fn test_creation_evenement() {
    let event_data = EventData {
        description: String::from("Test Event"),
        _target_id: Some(1),
    };
    
    let event = Evenement::new(1, 10, EventType::RandomEncounter, Some(event_data.clone()));

    assert_eq!(event._id, 1);
    assert_eq!(event.trigger_time, 10);
    assert_eq!(event.event_type, EventType::RandomEncounter);
    assert!(event.data.is_some());
    assert_eq!(event.data.unwrap().description, "Test Event");
}

#[test]
fn test_execution_evenement_sans_panique() {
    let event_data = EventData {
        description: String::from("Test Execution"),
        _target_id: Some(1),
    };
    
    let event = Evenement::new(2, 5, EventType::PoliceRaid, Some(event_data));
    
    // Préparer un monde minimal pour l'exécution
    let lieu = Lieu::new(0, "Test Place", "Un lieu pour les tests.", PlaceType::Safe, vec![]);
    let time_manager = GestionnaireDeTemps::new(0, 1440);
    let mut monde = Monde::new(vec![lieu], vec![], time_manager, vec![]);

    // Exécuter l'événement sans crash
    event.execute(&mut monde);
}

#[test]
fn test_evenement_est_bien_resolu() {
    let event = Evenement::new(
        3,
        0,
        EventType::ScheduledMeeting,
        Some(EventData {
            description: "Rencontre programmée".to_string(),
            _target_id: None,
        }),
    );

    let lieu = Lieu::new(0, "Test Lieu", "Lieu test.", PlaceType::Neutral, vec![]);
    let time_manager = GestionnaireDeTemps::new(0, 1440);
    let mut monde = Monde::new(vec![lieu], vec![], time_manager, vec![event]);

    assert_eq!(monde.events.len(), 1);

    // Simuler une mise à jour qui devrait résoudre l'événement
    monde.update();

    // Après la mise à jour, la liste des événements doit être vide
    assert_eq!(monde.events.len(), 0);
}

use std::io::{self, Write};
use crate::world::monde::Monde;
use crate::characters::joueur::Joueur;
use crate::characters::personnage::Personnage;
use crate::utils::types_enums::ItemType;
use crate::characters::personnage::Effect;
use crate::scenario::scenarios::ScenarioManager;

pub struct MoteurDeJeu {
    pub world: Monde,
    pub player: Joueur,
    pub is_running: bool,
    pub current_menu: MenuPrincipal,
    pub current_connected_places: Vec<usize>,
    pub history: Vec<String>,
    pub scenario_manager: Option<ScenarioManager>,
}

#[derive(PartialEq)]
pub enum MenuPrincipal {
    Accueil,
    SeDeplacer,
    Inventaire,
    Statut,
    Quitter,
    Banque,
    Scenario,
}

impl MoteurDeJeu {
    pub fn new(world: Monde, player: Joueur) -> Self {
        Self {
            world,
            player,
            is_running: true,
            current_menu: MenuPrincipal::Accueil,
            current_connected_places: Vec::new(),
            history: Vec::new(),
            scenario_manager: None,
        }
    }
    pub fn run(&mut self) {
        while self.is_running {
            self.render();
            let choix = self.lire_entree_utilisateur();
            self.gerer_entree_principale(choix);
            self.mettre_a_jour();
        }
    }
    fn render(&mut self) {
        self.afficher_interface_ascii();
        match self.current_menu {
            MenuPrincipal::Accueil => self.afficher_menu_accueil(),
            MenuPrincipal::SeDeplacer => self.afficher_menu_deplacement(),
            MenuPrincipal::Inventaire => self.afficher_menu_inventaire(),
            MenuPrincipal::Statut => self.afficher_menu_statut(),
            MenuPrincipal::Banque => self.afficher_menu_banque(),
            MenuPrincipal::Scenario => self.afficher_menu_scenario(),
            MenuPrincipal::Quitter => {},
        }
    }
    fn afficher_interface_ascii(&self) {
        let jour = self.world.time_manager.get_current_day();
        let heure = self.world.time_manager.get_current_hour();
        let minute = self.world.time_manager.get_current_minute();
        let lieu_actuel = self.world.get_place_name(self.player.current_place);
        let nom_joueur = self.player.get_name();
        print!("\x1B[2J\x1B[1;1H");
        println!("+-----------------------------------------------------------------------------------+");
        println!("|     O        | Nom : {:5}  | Force : {:3}/100                                     |", nom_joueur, self.player.power);
        println!("|    /|\\       | Sexe : {:6}           | Aura  : {:3}/100                          |", 
                 match self.player.sex { crate::utils::types_enums::Sex::Male => "Homme", crate::utils::types_enums::Sex::Female => "Femme", _ => "Autre" },
                 self.player.aura);
        println!("|    / \\       | Age : {:3}               | Argent : {:4}$                           |", self.player.age, self.player.money);
        println!("|              | Santé : {:3}/100         | Exp : {} xp                               |", self.player.health, self.player.experience);
        println!("|              | Faim  : {:03}/100         | Level : {}                                |", self.player.hunger, self.player.level);
        println!("+-----------------------------------------------------------------------------------+");
        println!("| Jour {} - {:02}:{:02}           Zone : « {} »", jour, heure, minute, lieu_actuel);
        println!("+-----------------------------------------------------------------------------------+");
        if self.current_menu == MenuPrincipal::Accueil {
            println!("1) Se déplacer");
            println!("2) Interagir avec le lieu");
            println!("3) Inventaire");
            println!("4) Statut (détails)");
            println!("5) Quitter");
            print!("\nEntrez votre choix : ");
            io::stdout().flush().unwrap();
        }
    }
    fn afficher_menu_accueil(&self) {}
    fn afficher_menu_deplacement(&mut self) {
        println!("");
        println!("MENU DÉPLACEMENT");
        self.current_connected_places.clear();
        if let Some(current_place) = self.world.find_place_by_id(self.player.current_place) {
            println!("Depuis {}, vous pouvez vous rendre ici :", current_place.name);
            for &place_id in &current_place.connected_places {
                if let Some(place) = self.world.find_place_by_id(place_id) {
                    self.current_connected_places.push(place_id);
                    let index = self.current_connected_places.len();
                    println!("{}) {}", index, place.name);
                }
            }
        }
        println!("0) Retour");
        print!("Choix : ");
        io::stdout().flush().unwrap();
    }
    fn afficher_menu_inventaire(&self) {
        println!("");
        println!("+------------------------- INVENTAIRE -------------------------+");
        if self.player.inventory.items.is_empty() {
            println!("|                      (Aucun objet)                          |");
        } else {
            for it in &self.player.inventory.items {
                println!("| > {} ({}$)                                                    |", it.name, it.value);
            }
        }
        println!("+--------------------------------------------------------------+");
        println!("0) Retour");
        print!("\nEntrez votre choix : ");
        io::stdout().flush().unwrap();
    }
    fn afficher_menu_statut(&self) {
        println!("");
        println!("--- STATUT DU JOUEUR ---");
        println!("Nom       : {}", self.player.get_name());
        println!("Santé     : {}/100", self.player.get_health());
        println!("Faim      : {}/100", self.player.get_hunger());
        println!("Argent    : {}$", self.player.get_money());
        println!("Banque    : {}$", self.player.bank_balance);
        println!("Niveau    : {}", self.player.level);
        println!("Expérience: {}", self.player.experience);
        println!("--------------------------------------------------------------");
        println!("Lieu actuel : {}", self.world.get_place_name(self.player.current_place));
        println!("--------------------------------------------------------------");
        println!("0) Retour au menu");
        print!("Choix : ");
        io::stdout().flush().unwrap();
    }
    fn afficher_menu_banque(&self) {
        println!("");
        println!("MENU BANQUE - GESTION DE COMPTE");
        println!("Solde en banque: {}$", self.player.bank_balance);
        println!("1) Déposer de l'argent");
        println!("2) Retirer de l'argent");
        println!("3) Examiner le lieu");
        println!("0) Retour");
        print!("\nEntrez votre choix : ");
        io::stdout().flush().unwrap();
    }
    fn get_scenario_file(&self) -> &str {
        match self.player.current_place {
            0 => "scenarios/home.xml",
            1 => "scenarios/city.xml",
            2 => "scenarios/bank.xml",
            3 => "scenarios/supermarche.xml",
            4 => "scenarios/neighborhood.xml",
            5 => "scenarios/plage.xml",
            6 => "scenarios/hospital.xml",
            7 => "scenarios/prison.xml",
            8 => "scenarios/gym.xml",
            _ => "scenarios/home.xml",
        }
    }
    fn afficher_menu_scenario(&mut self) {
        let file_path = self.get_scenario_file();
        if self.scenario_manager.is_none() {
            self.scenario_manager = Some(ScenarioManager::load_from_file(file_path));
        }
        if let Some(manager) = &self.scenario_manager {
            if let Some(scenario) = manager.get_current_scenario() {
                println!("");
                println!("INFO : {}", scenario.description);
                if scenario.choices.is_empty() {
                    println!("0) Retour");
                } else {
                    for (i, choice) in scenario.choices.iter().enumerate() {
                        println!("{}) {}", i + 1, choice.text);
                    }
                    println!("0) Retour");
                }
                print!("\nEntrez votre choix : ");
                io::stdout().flush().unwrap();
            }
        }
    }
    fn gerer_entree_principale(&mut self, input: String) {
        match self.current_menu {
            MenuPrincipal::Accueil => self.gerer_menu_accueil(input),
            MenuPrincipal::SeDeplacer => self.gerer_menu_deplacement(input),
            MenuPrincipal::Inventaire => self.gerer_menu_inventaire(input),
            MenuPrincipal::Statut => self.gerer_menu_statut(input),
            MenuPrincipal::Banque => self.gerer_menu_banque(input),
            MenuPrincipal::Scenario => self.gerer_menu_scenario(input),
            MenuPrincipal::Quitter => {},
        }
    }
    fn gerer_menu_accueil(&mut self, input: String) {
        match input.trim() {
            "1" => self.current_menu = MenuPrincipal::SeDeplacer,
            "2" => { self.current_menu = MenuPrincipal::Scenario; self.scenario_manager = None; },
            "3" => self.current_menu = MenuPrincipal::Inventaire,
            "4" => self.current_menu = MenuPrincipal::Statut,
            "5" => { self.current_menu = MenuPrincipal::Quitter; self.stop(); },
            _ => {}
        }
    }
    fn gerer_menu_deplacement(&mut self, input: String) {
        if let Ok(val) = input.trim().parse::<usize>() {
            if val == 0 {
                self.current_menu = MenuPrincipal::Accueil;
            } else if val <= self.current_connected_places.len() {
                let place_id = self.current_connected_places[val - 1];
                self.player.move_to(place_id);
                self.history.push(format!("Déplacement vers {}", self.world.get_place_name(place_id)));
                self.current_menu = MenuPrincipal::Accueil;
            }
        }
    }
    fn gerer_menu_inventaire(&mut self, input: String) {
        match input.trim() {
            "0" => self.current_menu = MenuPrincipal::Accueil,
            "1" => { self.history.push("Utilisation d'un objet (non implémenté)".to_string()); self.current_menu = MenuPrincipal::Accueil; },
            "2" => { self.history.push("Jet d'un objet (non implémenté)".to_string()); self.current_menu = MenuPrincipal::Accueil; },
            _ => { self.current_menu = MenuPrincipal::Accueil; }
        }
    }
    fn gerer_menu_statut(&mut self, input: String) {
        if input.trim() == "0" { self.current_menu = MenuPrincipal::Accueil; }
    }
    fn gerer_menu_banque(&mut self, input: String) {
        match input.trim() {
            "0" => self.current_menu = MenuPrincipal::Accueil,
            "1" => {
                print!("Montant à déposer : ");
                io::stdout().flush().unwrap();
                if let Some(val) = self.lire_somme_argent() {
                    self.player.deposit_money(val);
                    self.history.push(format!("Dépôt de {} $ en banque", val));
                }
                self.current_menu = MenuPrincipal::Accueil;
            },
            "2" => {
                print!("Montant à retirer : ");
                io::stdout().flush().unwrap();
                if let Some(val) = self.lire_somme_argent() {
                    self.player.withdraw_money(val);
                    self.history.push(format!("Retrait de {} $ de la banque", val));
                }
                self.current_menu = MenuPrincipal::Accueil;
            },
            "3" => { self.history.push("Examen du lieu : Banque".to_string()); self.current_menu = MenuPrincipal::Accueil; },
            _ => {}
        }
    }
    fn gerer_menu_scenario(&mut self, input: String) {
        if let Some(manager) = &mut self.scenario_manager {
            if let Some(scenario) = manager.get_current_scenario() {
                if scenario.choices.is_empty() || input.trim() == "0" {
                    self.current_menu = MenuPrincipal::Accueil;
                    manager.current_id = None;
                } else {
                    let choice_index = input.trim().parse::<usize>().unwrap_or(0);
                    if choice_index > 0 && choice_index <= scenario.choices.len() {
                        let selected_choice = &scenario.choices[choice_index - 1];
                        let next_id = selected_choice.id.clone();
                        manager.set_current_scenario(next_id);
                        manager.apply_effects(&mut self.player);
                        self.history.push("Scénario appliqué".to_string());
                        if let Some(new_scenario) = manager.get_current_scenario() {
                            if new_scenario.choices.is_empty() {
                                self.current_menu = MenuPrincipal::Accueil;
                                manager.current_id = None;
                            } else {
                                self.current_menu = MenuPrincipal::Scenario;
                            }
                        }
                    }
                }
            }
        }
    }
    fn gerer_menu_interactions(&mut self, _input: String) {}
    fn gerer_interactions_centreville(&mut self, _input: String) {}
    fn gerer_interactions_banlieue(&mut self, _input: String) {}
    fn gerer_interactions_supermarche(&mut self, _input: String) {}
    fn gerer_interactions_generiques(&mut self, _input: String) {}
    fn lire_somme_argent(&self) -> Option<i32> {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            if let Ok(val) = buffer.trim().parse::<i32>() {
                return Some(val);
            }
        }
        None
    }
    fn lire_entree_utilisateur(&self) -> String {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => buffer.trim().to_string(),
            Err(_) => String::new(),
        }
    }
    fn mettre_a_jour(&mut self) {
        self.world.update();
        if self.player.get_health() <= 0 {
            println!("Vous êtes mort. Fin de la partie.");
            self.stop();
        }
    }
    pub fn stop(&mut self) {
        self.is_running = false;
    }
}

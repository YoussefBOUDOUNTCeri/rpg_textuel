use std::fs::File;
use std::io::BufReader;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::characters::joueur::Joueur;


#[derive(Clone)]
pub struct Choice {
    pub id: String,
    pub text: String,
}

#[derive(Clone)]
pub struct Scenario {
    pub id: String,
    pub description: String,
    pub effects: Vec<String>,
    pub choices: Vec<Choice>,
}

pub struct ScenarioManager {
    pub scenarios: Vec<Scenario>,
    pub current_id: Option<String>,
}

impl ScenarioManager {
    pub fn load_from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let file = BufReader::new(file);
        let mut reader = Reader::from_reader(file);
        reader.trim_text(true);
        let mut buf = Vec::new();
        let mut scenarios = Vec::new();
        let mut current_scenario = Scenario {
            id: String::new(),
            description: String::new(),
            effects: Vec::new(),
            choices: Vec::new(),
        };
        let mut current_choice: Option<Choice> = None;
        let mut current_tag: Option<String> = None;
        let mut in_scenario = false;
        let mut in_effect = false;
        // let mut in_possible = false;
        let mut in_choice = false;
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    current_tag = Some(String::from_utf8_lossy(e.name().as_ref()).to_string());
                    match current_tag.as_deref() {
                        Some("scenario") => {
                            in_scenario = true;
                            current_scenario = Scenario {
                                id: String::new(),
                                description: String::new(),
                                effects: Vec::new(),
                                choices: Vec::new(),
                            }
                        }
                        Some("effect") => {
                            in_effect = true;
                        }
                        Some("possible_scenario_id") => {
                            // in_possible = true;
                        }
                        Some("choice") => {
                            in_choice = true;
                            current_choice = Some(Choice {
                                id: String::new(),
                                text: String::new(),
                            });
                        }
                        _ => {}
                    }
                },
                Ok(Event::End(ref e)) => {
                    let end_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match end_tag.as_str() {
                        "scenario" => {
                            in_scenario = false;
                            scenarios.push(current_scenario.clone());
                        }
                        "effect" => {
                            in_effect = false;
                        }
                        "possible_scenario_id" => {
                            // in_possible = false;
                        }
                        "choice" => {
                            in_choice = false;
                            if let Some(choice) = current_choice.take() {
                                current_scenario.choices.push(choice);
                            }
                        }
                        _ => {}
                    }
                    current_tag = None;
                },
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap().into_owned();
                    if in_scenario {
                        match current_tag.as_deref() {
                            Some("_id") => {
                                if current_scenario.id.is_empty() {
                                    current_scenario.id = text.clone();
                                }
                            }
                            Some("description") => {
                                if current_scenario.description.is_empty() {
                                    current_scenario.description = text.clone();
                                }
                            }
                            Some("e") if in_effect => {
                                current_scenario.effects.push(text.clone());
                            }
                            Some("id") if in_choice => {
                                if let Some(ref mut choice) = current_choice {
                                    choice.id = text.clone();
                                }
                            }
                            Some("text") if in_choice => {
                                if let Some(ref mut choice) = current_choice {
                                    choice.text = text.clone();
                                }
                            }
                            _ => {}
                        }
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("Error at position {}: {:?}", reader.buffer_position(), e);
                    break;
                },
                _ => {}
            }
            buf.clear();
        }
        ScenarioManager { scenarios, current_id: None }
    }

    pub fn get_current_scenario(&self) -> Option<&Scenario> {
        match &self.current_id {
            Some(id) => self.scenarios.iter().find(|s| s.id == *id),
            None => self.scenarios.first(),
        }
    }

    pub fn set_current_scenario(&mut self, id: String) {
        self.current_id = Some(id);
    }

    pub fn apply_effects(&self, joueur: &mut Joueur) {
        if let Some(scenario) = self.get_current_scenario() {
            for effect in &scenario.effects {
                let parts: Vec<&str> = effect.split_whitespace().collect();
                if parts.len() >= 3 {
                    let attr = parts[0];
                    let op = parts[1];
                    let val_str = parts[2].replace("%", "");
                    if let Ok(val) = val_str.parse::<i32>() {
                        match attr {
                            "health" => {
                                if op == "+" { joueur.health += val; } else { joueur.health -= val; }
                                if joueur.health > 100 { joueur.health = 100; }
                                if joueur.health < 0 { joueur.health = 0; }
                            },
                            "hunger" => {
                                if op == "+" { joueur.hunger += val; } else { joueur.hunger -= val; }
                                if joueur.hunger > 100 { joueur.hunger = 100; }
                                if joueur.hunger < 0 { joueur.hunger = 0; }
                            },
                            "power" => {
                                if op == "+" { joueur.power += val; } else { joueur.power -= val; }
                                if joueur.power < 0 { joueur.power = 0; }
                            },
                            "aura" => {
                                if op == "+" { joueur.aura += val; } else { joueur.aura -= val; }
                                if joueur.aura < 0 { joueur.aura = 0; }
                            },
                            "money" => {
                                if op == "+" { joueur.money += val; } else { joueur.money -= val; }
                                if joueur.money < 0 { joueur.money = 0; }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
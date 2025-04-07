#![allow(dead_code)]
pub struct DialogueChoice {
    pub text: String,
    pub next_line: usize,
}

pub struct Dialogue {
    pub npc_id: usize,
    pub lines: Vec<String>,
    pub current_line_index: usize,
    pub choices: Vec<DialogueChoice>,
}

impl Dialogue {
    pub fn new(npc_id: usize, lines: Vec<String>, choices: Vec<DialogueChoice>) -> Self {
        Self {
            npc_id,
            lines,
            current_line_index: 0,
            choices,
        }
    }

    pub fn start_dialogue(&mut self) {
        self.current_line_index = 0;
    }

    pub fn next_line(&mut self) {
        if self.current_line_index < self.lines.len() - 1 {
            self.current_line_index += 1;
        }
    }

    pub fn select_choice(&mut self, choice_index: usize) {
        if let Some(choice) = self.choices.get(choice_index) {
            if choice.next_line < self.lines.len() {
                self.current_line_index = choice.next_line;
            }
        }
    }
}
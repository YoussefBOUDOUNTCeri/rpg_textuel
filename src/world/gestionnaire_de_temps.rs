#![allow(dead_code)]
pub struct GestionnaireDeTemps {
    pub current_time: u32,
    pub day_length: u32,
}

impl GestionnaireDeTemps {
    pub fn new(current_time: u32, day_length: u32) -> Self {
        Self {
            current_time,
            day_length,
        }
    }

    pub fn advance_time(&mut self, amount: u32) {
        self.current_time += amount;
    }

    pub fn is_daytime(&self) -> bool {
        let cycle = self.current_time % self.day_length;
        cycle < (self.day_length / 2)
    }

    pub fn get_current_day(&self) -> u32 {
        self.current_time / self.day_length
    }

    pub fn get_current_hour(&self) -> u32 {
        let cycle = self.current_time % self.day_length;
        (cycle * 24) / self.day_length
    }

    pub fn get_current_minute(&self) -> u32 {
        let cycle = self.current_time % self.day_length;
        (cycle * 24 * 60 / self.day_length) % 60
    }
}
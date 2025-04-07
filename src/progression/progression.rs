#![allow(dead_code)]
use crate::utils::types_enums::PathType;

pub struct Progression {
    pub path_type: PathType,
    pub current_rank: i32,
    pub max_rank: i32,
}

impl Progression {
    pub fn new(path_type: PathType, current_rank: i32, max_rank: i32) -> Self {
        Self {
            path_type,
            current_rank,
            max_rank,
        }
    }

    pub fn advance(&mut self, amount: i32) {
        self.current_rank += amount;
        if self.current_rank > self.max_rank {
            self.current_rank = self.max_rank;
        }
    }

    pub fn regress(&mut self, amount: i32) {
        self.current_rank -= amount;
        if self.current_rank < 0 {
            self.current_rank = 0;
        }
    }
}
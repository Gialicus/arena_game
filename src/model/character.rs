use std::sync::{Arc, Mutex};

use crate::utils::pretty::print_damage_taken;

use super::skill::Skill;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub health: Arc<Mutex<i32>>,
    pub skills: Vec<Skill>,
}

impl Character {
    pub fn new(name: &str, health: i32, skills: Vec<Skill>) -> Self {
        Character {
            name: name.to_string(),
            health: Arc::new(Mutex::new(health)),
            skills,
        }
    }

    pub fn take_damage(&self, amount: i32) {
        let mut health = self.health.lock().unwrap();
        *health -= amount;
        print_damage_taken(&self.name, amount, *health);
    }

    pub fn is_alive(&self) -> bool {
        let health = self.health.lock().unwrap();
        *health > 0
    }
}

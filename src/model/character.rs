use std::{
    fmt::{self},
    sync::{Arc, Mutex},
};

use crate::utils::pretty::print_damage_taken;

use super::skill::Skill;

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub health: Arc<Mutex<i32>>,
    pub skills: Vec<Skill>,
}

impl Character {
    pub fn new(name: &str, skills: Vec<Skill>) -> Self {
        Character {
            name: name.to_string(),
            health: Arc::new(Mutex::new((skills.len() * 100) as i32)),
            skills,
        }
    }

    pub fn level_up(&self, skills: Vec<Skill>) -> Self {
        Character::new(
            &self.name,
            self.skills.iter().chain(skills.iter()).cloned().collect(),
        )
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

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {},\nHealth: {},\nSkills: {}",
            self.name,
            *self.health.lock().unwrap(),
            self.skills
                .clone()
                .into_iter()
                .map(|s| format!(
                    "{} {}-{} {}s",
                    s.name,
                    s.damage.0,
                    s.damage.1,
                    s.cooldown.as_secs().to_string()
                ))
                .collect::<Vec<String>>()
                .join(",\n")
        )
    }
}

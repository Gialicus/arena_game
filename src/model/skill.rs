use std::time::Duration;

use super::element::Element;

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub damage: (i32, i32),
    pub cooldown: Duration,
    pub element: Element,
}

impl Skill {
    pub fn new(name: &str, damage: (i32, i32), cooldown_secs: u64, element: Element) -> Self {
        Skill {
            name: name.to_string(),
            damage,
            cooldown: Duration::from_secs(cooldown_secs),
            element,
        }
    }
}

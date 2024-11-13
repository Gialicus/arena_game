use colored::{ColoredString, Colorize};

use crate::model::{element::Element, skill::Skill};

pub fn print_by_element(txt: &str, element: &Element) -> ColoredString {
    match element {
        Element::Water => txt.blue().bold(),
        Element::Fire => txt.red().bold(),
        Element::Earth => txt.green().bold(),
        Element::Air => txt.yellow().bold(),
    }
}

pub fn print_damage(attacker: &str, skill: &Skill, damage: i32) {
    let skill_name = print_by_element(&skill.name, &skill.element);
    let damage = format!("{}", damage);
    let damage = damage.red().bold();
    println!("{} usa {} (danno: {})", attacker, skill_name, damage);
}

pub fn print_damage_taken(name: &str, amount: i32, health: i32) {
    println!(
        "{} subisce {} danni, salute rimanente: {}",
        name,
        amount.to_string().red(),
        health.to_string().green()
    );
}

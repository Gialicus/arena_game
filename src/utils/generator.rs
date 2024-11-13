use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

use crate::model::{character::Character, cooldown::COOLDOWNS, element::Element, skill::Skill};

fn name_generator(element: &Element, rng: &mut ThreadRng) -> String {
    let prefixes = [
        "Hit", "Arrow", "Blade", "Beam", "Ball", "Bash", "Strike", "Slash", "Bolt", "Punch",
        "Kick", "Smash",
    ];
    let suffixes = [
        "of Doom",
        "of Pain",
        "of Fury",
        "of Chaos",
        "of Destruction",
        "of Darkness",
        "of Light",
        "of Thunder",
        "of Storms",
        "of Shadows",
    ];
    let prefix = prefixes.choose(rng).unwrap();
    let suffix = suffixes.choose(rng).unwrap();
    format!("{} {} {}", element, prefix, suffix)
}

fn generate_cd_and_damage(rng: &mut ThreadRng) -> (i32, (i32, i32)) {
    let cooldown = COOLDOWNS.choose(rng).unwrap();
    match cooldown {
        3 => (3, (15, 20)),
        5 => (5, (30, 50)),
        8 => (8, (50, 70)),
        _ => (0, (0, 0)),
    }
}

pub fn generate_random_skill() -> Skill {
    let mut rng = thread_rng();
    let elements = [Element::Water, Element::Fire, Element::Earth, Element::Air];
    let element = elements.choose(&mut rng).unwrap();
    let (cooldown, damage) = generate_cd_and_damage(&mut rng);
    Skill::new(
        name_generator(&element, &mut rng).as_str(),
        damage,
        cooldown as u64,
        element.clone(),
    )
}

pub fn shuffle_characters(characters: Vec<Character>) -> Vec<Character> {
    let mut rng = thread_rng();
    let mut characters = characters;
    characters.shuffle(&mut rng);
    characters
}

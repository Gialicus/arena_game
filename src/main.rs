mod model;
mod utils;

use colored::Colorize;
use model::character::Character;
use model::fight::start_fight;
use utils::generator::generate_random_skill;

#[tokio::main]
async fn main() {
    // Inizializzazione dei personaggi e delle skill
    let skills_player1 = vec![
        generate_random_skill(),
        generate_random_skill(),
        generate_random_skill(),
        generate_random_skill(),
    ];

    let skills_player2 = vec![
        generate_random_skill(),
        generate_random_skill(),
        generate_random_skill(),
        generate_random_skill(),
    ];

    let war = Character::new("Guerriero", skills_player1);
    let archer = Character::new("Arciere", skills_player2);
    let winner = start_fight(war, archer).await;
    let end_title = format!("\nThe winner is \n****\n{}\n****\n", winner).cyan();
    println!("{}", end_title)
}

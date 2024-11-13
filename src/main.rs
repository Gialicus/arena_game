mod model;
mod utils;

use std::sync::Arc;

use model::character::Character;
use model::fight::character_attack_loop;
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

    let player1 = Arc::new(Character::new("Guerriero", 400, skills_player1));
    let player2 = Arc::new(Character::new("Arciere", 400, skills_player2));

    // Avvia le task di combattimento per entrambi i personaggi
    let player1_task = character_attack_loop(player1.clone(), player2.clone());
    let player2_task = character_attack_loop(player2.clone(), player1.clone());

    tokio::join!(player1_task, player2_task);

    // Determina il vincitore
    if player1.is_alive() {
        println!("{} vince!", player1.name);
    } else {
        println!("{} vince!", player2.name);
    }
}

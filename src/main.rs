mod model;
mod utils;

use model::arena::start_tournament;
use model::character::Character;
use utils::generator::generate_random_skill;

#[tokio::main]
async fn main() {
    // Inizializzazione dei personaggi e delle skill
    let skills_player1 = vec![generate_random_skill()];

    let skills_player2 = vec![generate_random_skill()];

    let skills_player3 = vec![generate_random_skill()];

    let skills_player4 = vec![generate_random_skill()];

    let war = Character::new("Guerriero", skills_player1);
    let archer = Character::new("Arciere", skills_player2);
    let rogue = Character::new("Ladro", skills_player3);
    let mage = Character::new("Mago", skills_player4);
    println!("\n*** START TOURNAMENT ***\n");
    let winner = start_tournament(vec![war, archer, rogue, mage]).await;
    println!("\n*** WINNER ***\n{}\n", winner)
}

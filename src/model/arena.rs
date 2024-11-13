use crate::{model::fight::start_fight, utils::generator::generate_random_skill};

use super::character::Character;

pub async fn start_round(players: Vec<Character>) -> Vec<Character> {
    if players.len() == 1 {
        return players;
    }
    if players.len() % 2 != 0 {
        panic!("Players cant not be Odd")
    }

    let mut teams = vec![];
    for i in 0..players.len() / 2 {
        let team = (players[i * 2].clone(), players[i * 2 + 1].clone());
        teams.push(team);
    }

    let mut round_winners = vec![];
    for (player_one, player_two) in teams {
        let winner = start_fight(player_one, player_two).await;
        round_winners.push(winner);
    }
    round_winners
}

pub async fn start_tournament(players: Vec<Character>) -> Character {
    let first_winners = start_round(players).await;
    if first_winners.len() == 1 {
        return first_winners.first().unwrap().clone();
    }
    let mut next_round_players = vec![];
    for first_winner in first_winners {
        println!("Round winners:\n{}\n", first_winner);
        next_round_players.push(first_winner.level_up(vec![generate_random_skill()]));
    }
    let result = Box::pin(start_tournament(next_round_players)).await;
    result
}

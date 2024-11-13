use std::sync::Arc;

use futures::future::join_all;

use crate::{model::fight::start_fight, utils::generator::generate_random_skill};

use super::character::Character;

pub async fn start_round(players: Vec<Character>) -> Vec<Character> {
    // Ritorna il singolo giocatore se c'è solo un giocatore (vincitore del torneo)
    if players.len() == 1 {
        return players;
    }

    // Crea le coppie di combattimento
    let mut teams = vec![];
    for i in 0..players.len() / 2 {
        let team = (players[i * 2].clone(), players[i * 2 + 1].clone());
        teams.push(team);
    }

    // Avvia i combattimenti in parallelo
    let mut handles = vec![];
    for (player_one, player_two) in teams {
        handles.push(start_fight(Arc::new(player_one), Arc::new(player_two)));
    }

    let round_winners = join_all(handles).await;
    round_winners.iter().map(|c| c.as_ref().clone()).collect()
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

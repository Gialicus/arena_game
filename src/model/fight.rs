use std::{ops::Deref, sync::Arc};

use rand::{seq::IteratorRandom, thread_rng};
use tokio::{sync::mpsc, time};

use crate::utils::pretty::print_damage;

use super::{character::Character, skill::Skill};

async fn skill_loop(skill: Skill, sender: mpsc::Sender<Skill>) {
    loop {
        time::sleep(skill.cooldown).await;
        if sender.send(skill.clone()).await.is_err() {
            break; // Termina se il ricevente non è più in ascolto
        }
    }
}

async fn character_attack_loop(character: Arc<Character>, opponent: Arc<Character>) {
    // Creiamo un canale per ricevere le notifiche delle skill pronte.
    let (tx, mut rx) = mpsc::channel(character.skills.len() * 2);
    let skills = character.skills.clone();
    // Avvia un task per ciascuna skill che invia notifiche al canale quando sono pronte
    for skill in skills {
        let tx_clone = tx.clone();
        tokio::spawn(skill_loop(skill, tx_clone));
    }
    let mut rng = thread_rng();

    // Loop principale: ascolta le notifiche dal canale e attacca quando una skill è pronta
    while character.is_alive() && opponent.is_alive() {
        if let Some(skill) = rx.recv().await {
            let (low, high) = skill.damage;
            let damage = (low..=high).choose(&mut rng).unwrap();
            print_damage(&character.name, &skill, damage);
            opponent.take_damage(damage);

            // Termina il combattimento se l'avversario è morto
            if !opponent.is_alive() {
                break;
            }
        }
    }
}

pub async fn start_fight(player_one: Character, player_two: Character) -> Character {
    let player1 = Arc::new(player_one);
    let player2 = Arc::new(player_two);

    // Avvia le task di combattimento per entrambi i personaggi
    let player1_task = character_attack_loop(player1.clone(), player2.clone());
    let player2_task = character_attack_loop(player2.clone(), player1.clone());

    tokio::join!(player1_task, player2_task);
    // Determina il vincitore
    if player1.is_alive() {
        println!("{} win!", player1.name);
        player1.deref().clone()
    } else {
        println!("{} win!", player2.name);
        player2.deref().clone()
    }
}

use crate::date::{Date, Game, Player};

pub fn create_date_sept_2021() -> Date {
    let mut date = Date::new(2021, 9);

    let games = vec![
        "Kjærleiksbrev",
        "Ølmesternes mester",
        "The Amazing Labyrinth",
        "Darts",
        "7 shots of glory",
        "Blindtest",
        "NattBoccia",
    ];

    for game in &games {
        date.add_game(Game::new(game))
    }

    let players = vec![
        ("Joakim", vec![2, 3, 2, 4, 3, 2, 5]),
        ("Bendik", vec![1, 6, 4, 5, 6, 1, 3]),
        ("Peter", vec![7, 2, 6, 2, 5, 3, 2]),
        ("Roar", vec![5, 5, 3, 1, 2, 6, 6]),
        ("Morten", vec![3, 4, 5, 3, 1, 5, 7]),
        ("Are", vec![4, 1, 7, 6, 4, 4, 4]),
        ("Trond", vec![6, 7, 1, 7, 7, 7, 1]),
    ];

    for (name, scores) in players {
        let player = Player::new(name);
        date.add_player(player);
        for (game, score) in games.iter().zip(scores.iter()) {
            date.set_score(name, game, *score);
        }
    }

    date
}

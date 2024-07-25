use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    title: String,
}

#[derive(Debug)]
pub struct Player {
    name: String,
}

#[derive(Debug)]
pub struct Date {
    pub year: u32,
    pub month: u8,
    pub games: HashMap<String, usize>, // Game title to column index mapping
    pub players: HashMap<String, usize>, // Player name to row index mapping
    pub scores: Vec<Vec<Option<u32>>>, // Matrix of scores
}

impl Game {
    pub fn new(title: &str) -> Self {
        Game {
            title: title.to_string(),
        }
    }
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
        }
    }
}

impl Date {
    pub fn new(year: u32, month: u8) -> Self {
        Date {
            year,
            month,
            games: HashMap::new(),
            players: HashMap::new(),
            scores: Vec::new(),
        }
    }

    pub fn add_game(&mut self, game: Game) {
        let col_index = self.games.len();
        self.games.insert(game.title.clone(), col_index);

        // Add a new column to the scores matrix
        for row in self.scores.iter_mut() {
            row.push(None);
        }
    }

    pub fn add_player(&mut self, player: Player) {
        let row_index = self.players.len();
        self.players.insert(player.name.clone(), row_index);

        // Add a new row to the scores matrix
        let cols = self.games.len();
        self.scores.push(vec![None; cols]);
    }

    pub fn set_score(&mut self, player_name: &str, game_title: &str, score: u32) {
        if let Some(&row_index) = self.players.get(player_name) {
            if let Some(&col_index) = self.games.get(game_title) {
                self.scores[row_index][col_index] = Some(score);
            } else {
                println!("Game '{}' not found.", game_title);
            }
        } else {
            println!("Player '{}' not found.", player_name);
        }
    }

    pub fn get_scores(&self) -> Vec<Vec<Option<u32>>> {
        self.scores.clone()
    }
}

mod date;
mod dates;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::Window;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let date_september = dates::sept_2021::create_date_sept_2021();
    let scores = date_september.get_scores();
    let games: Vec<_> = date_september.games.keys().cloned().collect();
    let players: Vec<_> = date_september.players.keys().cloned().collect();

    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let table = document.create_element("table").unwrap();
    table.set_attribute("border", "1").unwrap();

    // Create header row
    let header_row = document.create_element("tr").unwrap();
    let th = document.create_element("th").unwrap();
    th.set_inner_html("Deltager");
    header_row.append_child(&th).unwrap();
    for game in &games {
        let th = document.create_element("th").unwrap();
        th.set_inner_html(game);
        header_row.append_child(&th).unwrap();
    }
    table.append_child(&header_row).unwrap();

    // Create rows for players and their scores
    for player in players {
        let row = document.create_element("tr").unwrap();
        let td = document.create_element("td").unwrap();
        td.set_inner_html(&player);
        row.append_child(&td).unwrap();

        if let Some(&row_index) = date_september.players.get(&player) {
            for game in &games {
                let td = document.create_element("td").unwrap();
                if let Some(&col_index) = date_september.games.get(game) {
                    let score = scores[row_index][col_index];
                    td.set_inner_html(&format!("{:?}", score.unwrap_or(0)));
                }
                row.append_child(&td).unwrap();
            }
        }
        table.append_child(&row).unwrap();
    }

    body.append_child(&table).unwrap();
}

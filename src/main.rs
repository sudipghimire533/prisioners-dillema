use std::time::SystemTime;

pub mod game;
pub mod models;
pub mod ui;

pub struct PlayerScores {
    pub player_one_score: game::scores::DefaultScores,
    pub player_two_score: game::scores::DefaultScores,
}

const GAME_CONFIG: game::game::GameConfig = game::game::GameConfig {
    max_rounds: Some(10),
    outcome_buf_size: 100,
};

fn main() {
    // 1. Create two players
    let tit_tat = models::tit_for_tat::TitForTat;
    let devil = models::push_overs::AlwaysDefect;

    // 2.Create a game and simulate play
    let mut tit_tat_and_devil_game = game::game::Game::new();
    tit_tat_and_devil_game.play(&GAME_CONFIG, tit_tat, devil);

    // 3.Create a UI
    let ui_result = ui::ModelComparision {
        player_one: (
            "Tit for Tat".to_string(),
            tit_tat_and_devil_game.player_one_points,
        ),
        player_two: (
            "Devil".to_string(),
            tit_tat_and_devil_game.player_two_points,
        ),
        total_rounds: GAME_CONFIG.max_rounds.unwrap(),
    };

    // 4. Write a ui in a temporary html file
    let dest_file_name = format!(
        "prisioners_dillems_{}.html",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let mut temp_file_path = std::env::temp_dir();
    temp_file_path.push(dest_file_name);
    ui_result
        .draw_html(temp_file_path.as_path())
        .expect("Cannot draw result chart in temporary html file");

    // 5. open the written file in default browser
    webbrowser::open(temp_file_path.as_path().as_os_str().to_str().unwrap())
        .expect("Cannot open the file in browser. Please open manually: {temp_file_path}");
}

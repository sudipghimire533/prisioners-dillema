pub mod game;
pub mod models;

fn main() {
    let tit_tat = models::tit_for_tat::TitForTat;
    let devil = models::extremists::AlwaysDefect;

    let game_config = game::game::GameConfig {
        max_rounds: Some(10),
    };

    let mut tit_tat_and_devil_game = game::game::Game::new();
    tit_tat_and_devil_game.play(&game_config, tit_tat, devil);
}

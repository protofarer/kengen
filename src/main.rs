#[allow(warnings, dead_code)]
use kengen::game::Game;

fn main() {
    let mut game = Game::new().unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    game.run();
    game.destroy();
}

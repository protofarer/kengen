use kengen::ecs::components::HealthComponent;
#[allow(warnings, dead_code)]
use kengen::game::Game;

fn main() {
    // let mut game = Game::new().unwrap_or_else(|e| {
    //     println!("{e}");
    //     std::process::exit(1);
    // });

    // game.run();
    // game.destroy();
    let mut c = HealthComponent::new(None, None);
    c += 640000;
    println!("{}", c.hp);
}

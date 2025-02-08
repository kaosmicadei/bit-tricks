use libgame::Game;

fn main() {
  let game_status = (0..=6).into_iter()
    .fold(Game::new(), |game, m| {
      game.make_move(m).unwrap()
    });
    println!("{}", game_status);
}

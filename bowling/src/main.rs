use bowling::BowlingGame;

fn main() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..10 {
        let _ = game.roll(0);
    }
    let _ = game.roll(3);
    let _ = game.roll(7);
    let _ = game.roll(2);
    
    println!("{:?}", game);  
}
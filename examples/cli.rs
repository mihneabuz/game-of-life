use game_of_life::GameOfLife;

const WIDTH: usize = 100;
const HEIGHT: usize = 40;

const START: &[(usize, usize)] = &[
    (1, 5),
    (1, 6),
    (2, 5),
    (2, 6),
    (13, 3),
    (14, 3),
    (12, 4),
    (16, 4),
    (11, 5),
    (17, 5),
    (11, 6),
    (15, 6),
    (17, 6),
    (18, 6),
    (11, 7),
    (17, 7),
    (12, 8),
    (16, 8),
    (13, 9),
    (14, 9),
    (21, 3),
    (22, 3),
    (21, 4),
    (22, 4),
    (21, 5),
    (22, 5),
    (23, 2),
    (23, 6),
    (25, 1),
    (25, 2),
    (25, 6),
    (25, 7),
    (35, 3),
    (36, 3),
    (35, 4),
    (36, 4),
];

fn main() {
    let mut game = GameOfLife::new(WIDTH, HEIGHT);

    for pos in START {
        game.set(pos.0, pos.1, true);
    }

    for _ in 0..1000 {
        display(&game);
        std::thread::sleep(std::time::Duration::from_millis(100));
        game.step();
    }
}

fn display(game: &GameOfLife) {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if game.get(x, y).unwrap() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

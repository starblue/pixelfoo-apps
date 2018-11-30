use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat_with;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use pixelfoo::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Grass,
    Rabbit,
    Fox,
}

type Board = Vec<Vec<Square>>;

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for line in board {
        for square in line {
            let c = match square {
                Square::Empty => Color::blue(),
                Square::Grass => Color::green(),
                Square::Rabbit => Color::yellow(),
                Square::Fox => Color::red(),
            };
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

const DEFAULT_ARG: usize = 10;

fn grow(board: &Board, x: usize, y: usize, grow: Square, die: Square) -> Square {
    let x_size = board[0].len() as isize;
    let y_size = board.len() as isize;
    let x = x as isize;
    let y = y as isize;
    for (dx, dy) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let x1 = x + dx;
        let y1 = y + dy;
        if 0 <= x1 && x1 < x_size && 0 <= y1 && y1 < y_size {
            let neigh_sq = board[y1 as usize][x1 as usize];
            if neigh_sq == grow {
                return grow;
            }
        }
    }
    die
}

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = args[3].parse::<usize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();

    let p_empty = 0.25;
    let p_grass = 0.25;
    let p_rabbit = 0.25;
    // p_fox = 0.05
    let mut board = repeat_with(|| {
        repeat_with(|| {
            let p = rng.gen::<f64>();
            if p < p_empty {
                Square::Empty
            } else if p < p_empty + p_grass {
                Square::Grass
            } else if p < p_empty + p_grass + p_rabbit {
                Square::Rabbit
            } else {
                Square::Fox
            }
        })
        .take(x_size)
        .collect::<Vec<_>>()
    })
    .take(y_size)
    .collect::<Vec<_>>();

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let x_mid = (x_size - 1) as f64 / 2.0;
    let y_mid = (y_size - 1) as f64 / 2.0;
    let r = (x_size.min(y_size) - 1) as f64 / 2.5;
    loop {
        for _ in 0..arg {
            let x = rng.gen_range(0, x_size);
            let y = rng.gen_range(0, y_size);
            let sq = board[y][x];
            let dx = (x as f64 - x_mid as f64) / r as f64;
            let dy = (y as f64 - y_mid as f64) / r as f64;
            let p0 = (dx * dx + dy * dy).min(1.0);
            let p_survive = match sq {
                Square::Empty => 1.0,
                Square::Grass => p0,
                Square::Rabbit => p0,
                Square::Fox => 0.8 * p0,
            };
            let new_sq = if rng.gen::<f64>() < p_survive {
                sq
            } else {
                Square::Empty
            };
            board[y][x] = match sq {
                Square::Empty => grow(&board, x, y, Square::Grass, new_sq),
                Square::Grass => grow(&board, x, y, Square::Rabbit, new_sq),
                Square::Rabbit => grow(&board, x, y, Square::Fox, new_sq),
                Square::Fox => new_sq,
            };
        }
        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

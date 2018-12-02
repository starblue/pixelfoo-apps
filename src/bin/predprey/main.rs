use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat_with;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use pixelfoo::color::Color;
use pixelfoo::point2d::Point2d;
use pixelfoo::rect2d::Rect2d;
use pixelfoo::vec2d::v2d;
use pixelfoo::vec2d::Vec2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Grass,
    Rabbit,
    Fox,
}

struct Board(Vec<Vec<Square>>);

impl Board {
    pub fn new(size: Vec2d, mut f: impl FnMut() -> Square) -> Board {
        Board(
            repeat_with(|| {
                repeat_with(&mut f)
                    .take(size.x as usize)
                    .collect::<Vec<_>>()
            })
            .take(size.y as usize)
            .collect::<Vec<_>>(),
        )
    }
    pub fn size(&self) -> Vec2d {
        let x_size = self.0[0].len() as i32;
        let y_size = self.0.len() as i32;
        v2d(x_size, y_size)
    }
    pub fn rect(&self) -> Rect2d {
        Rect2d::new(0, self.size().x, 0, self.size().y)
    }
    pub fn get(&self, pos: Point2d) -> Square {
        self.0[pos.y as usize][pos.x as usize]
    }
    pub fn set(&mut self, pos: Point2d, sq: Square) {
        self.0[pos.y as usize][pos.x as usize] = sq;
    }
}

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    let Board(lines) = board;
    for line in lines {
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

fn grow(board: &Board, pos: Point2d, grow: Square, die: Square) -> Square {
    for pos1 in pos.neighbours() {
        if board.rect().contains(pos1) {
            let neigh_sq = board.get(pos1);
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
    let size = v2d(x_size as i32, y_size as i32);

    let p_empty = 0.25;
    let p_grass = 0.25;
    let p_rabbit = 0.25;
    // p_fox = 0.05
    let mut board = Board::new(size, || {
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
    });

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    // mid point of the board
    let x_mid = (x_size - 1) as f64 / 2.0;
    let y_mid = (y_size - 1) as f64 / 2.0;

    // radius of the death zone in the middle
    let r = (x_size.min(y_size) - 1) as f64 / 2.5;

    loop {
        for _ in 0..arg {
            let pos = board.rect().random_point(&mut rng);
            let sq = board.get(pos);
            let dx = (pos.x as f64 - x_mid as f64) / r as f64;
            let dy = (pos.y as f64 - y_mid as f64) / r as f64;
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
            board.set(
                pos,
                match sq {
                    Square::Empty => grow(&board, pos, Square::Grass, new_sq),
                    Square::Grass => grow(&board, pos, Square::Rabbit, new_sq),
                    Square::Rabbit => grow(&board, pos, Square::Fox, new_sq),
                    Square::Fox => new_sq,
                },
            );
        }

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

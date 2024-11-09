use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo_apps::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Grass,
    Rabbit,
    Fox,
}

#[derive(Clone, Debug)]
struct Board(Array2d<i64, Square>);
impl Board {
    pub fn new(bbox: BBox2d, mut f: impl FnMut() -> Square) -> Board {
        Board(Array2d::with(bbox, |_p| f()))
    }
    fn bbox(&self) -> BBox2d {
        self.0.bbox()
    }
    fn get(&self, pos: Point2d) -> Square {
        self.0[pos]
    }
    fn set(&mut self, pos: Point2d, sq: Square) {
        self.0[pos] = sq;
    }
}

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for y in board.bbox().y_range() {
        for x in board.bbox().x_range() {
            let square = board.0[p2d(x, y)];
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

fn grow(board: &Board, pos: Point2d, grow: Square, die: Square) -> Square {
    for pos1 in pos.neighbors_l1() {
        if board.bbox().contains(&pos1) {
            let neigh_sq = board.get(pos1);
            if neigh_sq == grow {
                return grow;
            }
        }
    }
    die
}

const DEFAULT_ARG: usize = 10;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<i64>().unwrap();
    let y_size = args[2].parse::<i64>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<usize>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();
    let bbox = bb2d(0..x_size, 0..y_size);

    let p_empty = 0.25;
    let p_grass = 0.25;
    let p_rabbit = 0.25;
    let mut board = Board::new(bbox, || {
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
    let x_mid = (bbox.x_min() + bbox.x_max()) as f64 / 2.0;
    let y_mid = (bbox.y_min() + bbox.y_max()) as f64 / 2.0;

    // radius of the death zone in the middle
    let r = (x_size.min(y_size) - 1) as f64 / 2.5;

    loop {
        for _ in 0..arg {
            let pos = bbox.random_point(&mut rng);
            let sq = board.get(pos);
            let dx = (pos.x() as f64 - x_mid as f64) / r as f64;
            let dy = (pos.y() as f64 - y_mid as f64) / r as f64;
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

        let mut buf = Vec::with_capacity(bbox.area() * 3);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

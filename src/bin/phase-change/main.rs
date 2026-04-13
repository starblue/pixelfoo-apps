use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::rng;
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
    Full,
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
                Square::Empty => Color::new(0, 0, 128),
                Square::Full => Color::new(220, 180, 80),
            };
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

const DEFAULT_ARG: usize = 60;

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

    let mut rng = rng();
    let bbox = bb2d(0..x_size, 0..y_size);

    let p_full = 0.25;
    let mut board = Board::new(bbox, || {
        let p = rng.random::<f64>();
        if p < p_full {
            Square::Full
        } else {
            Square::Empty
        }
    });

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let actions_per_frame = 200;

    // Temperature
    let temp_min = 0.0;
    let temp_max = 1.5;

    // Period for temperature cycle in seconds.
    let t_period = arg as f64;

    // Temperature change per frame.
    let d_temp = 2.0 * (temp_max - temp_min) / (t_period / t_frame);

    let mut temp = temp_max;
    let mut heating = false;
    loop {
        if heating && temp > temp_max {
            heating = false;
        } else if !heating && temp < temp_min {
            heating = true;
        }

        if heating {
            temp += d_temp;
        } else {
            temp -= d_temp;
        }

        for _ in 0..actions_per_frame {
            let x0 = rng.random_range(bbox.x_range());
            let y0 = rng.random_range(bbox.y_range());
            let pos0 = p2d(x0, y0);

            let x1 = rng.random_range(bbox.x_range());
            let y1 = rng.random_range(bbox.y_range());
            let pos1 = p2d(x1, y1);

            let sq0 = board.get(pos0);
            let sq1 = board.get(pos1);
            if sq0 != sq1 {
                let count0 = pos0
                    .neighbors_l1()
                    .filter(|&np| bbox.contains(&np) && board.get(np) == Square::Full)
                    .count() as i8;
                let count1 = pos1
                    .neighbors_l1()
                    .filter(|&np| bbox.contains(&np) && board.get(np) == Square::Full)
                    .count() as i8;

                // The energy change if we swap `sq0` and `sq1`.
                // For each pair of orthogonally adjacent full squares
                // we count `-1` for the total energy.
                let delta = if sq0 == Square::Full {
                    count1 - count0
                } else {
                    count0 - count1
                } as f64;

                let q = (delta / temp).exp();
                let p = q / (1.0 + q);

                if rng.random::<f64>() < p {
                    // Actually do the swap.
                    board.set(pos1, sq0);
                    board.set(pos0, sq1);
                }
            }
        }

        let mut buf = Vec::with_capacity(bbox.area() * 3);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

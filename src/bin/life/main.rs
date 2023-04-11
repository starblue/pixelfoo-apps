use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Full,
}

#[derive(Clone, Debug)]
struct Board {
    /// The inner bounding box that should be displayed.
    bbox: BBox2d,
    map: Array2d<i64, Square>,
}
impl Board {
    pub fn with(bbox: BBox2d, f: impl FnMut(Point2d) -> Square) -> Board {
        let new_bbox = BBox2d::from_corners(bbox.min() - v2d(1, 1), bbox.max() + v2d(1, 1));
        // The actual map is enlarged by one square on all sides
        // to avoid special cases when regarding the neighbors.
        // The boundary squares can be filled with some data.
        // For example they can stay empty or be filled randomly.
        let map = Array2d::with(new_bbox, f);
        Board { bbox, map }
    }
    pub fn bbox(&self) -> BBox2d {
        self.bbox
    }
    pub fn get(&self, pos: Point2d) -> Square {
        self.map[pos]
    }
}

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for y in board.bbox().y_range() {
        for x in board.bbox().x_range() {
            let square = board.map[p2d(x, y)];
            let c = match square {
                Square::Empty => Color::black(),
                Square::Full => Color::green(),
            };
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

const DEFAULT_ARG: usize = 10;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = args[3].parse::<usize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();
    let bbox = bb2d(0..x_size as i64, 0..y_size as i64);

    let p_full = 0.25;
    let mut board = Board::with(bbox, |_p| {
        // Initialize board randomly.
        let r = rng.gen::<f64>();
        if r < p_full {
            Square::Full
        } else {
            Square::Empty
        }
    });

    let t_frame = 0.5; // s
    let delay = Duration::from_secs_f64(t_frame);

    loop {
        let new_board = Board::with(bbox, |p| {
            if bbox.contains(&p) {
                // Use the game of life rule in the interior.
                let nc = p
                    .neighbors_l_infty()
                    .filter(|&np| board.get(np) == Square::Full)
                    .count();
                if nc == 3 || (nc == 2 && board.get(p) == Square::Full) {
                    Square::Full
                } else {
                    Square::Empty
                }
            } else {
                if p.y() < bbox.y_min() || p.y() > bbox.y_max() {
                    // The invisible long edge is filled randomly.
                    let r = rng.gen::<f64>();
                    if r < p_full {
                        Square::Full
                    } else {
                        Square::Empty
                    }
                } else {
                    // The short edges are empty.
                    Square::Empty
                }
            }
        });
        board = new_board;

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

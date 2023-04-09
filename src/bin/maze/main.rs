use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use chrono::Timelike;

use rand::thread_rng;
use rand::Rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;
use lowdim::Vector;

use pixelfoo::color::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Unused,
    Unknown { prio: i32 },
    Corridor,
    Wall,
    Start,
    Finish,
}

impl Square {
    pub fn is_unknown(&self) -> bool {
        match self {
            Square::Unknown { .. } => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct Board(Array2d<i64, Square>);
impl Board {
    fn bbox(&self) -> BBox2d {
        self.0.bbox()
    }
}

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for y in board.bbox().y_range() {
        for x in board.bbox().x_range() {
            let square = board.0[p2d(x, y)];
            let c = match square {
                Square::Unused => Color::black(),
                Square::Unknown { prio } => {
                    if prio == 0 {
                        Color::black()
                    } else if prio < 0 {
                        Color::lightblue()
                    } else {
                        Color::darkyellow()
                    }
                }
                Square::Corridor => Color::yellow(),
                Square::Wall => Color::darkblue(),
                Square::Start => Color::red(),
                Square::Finish => Color::green(),
            };
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Board {
    fn new(board_bbox: BBox2d, maze_bbox: BBox2d) -> Board {
        Board(Array2d::with(board_bbox, |p| {
            if maze_bbox.contains(&p) {
                let x = p.x();
                let y = p.y();
                if x == maze_bbox.x_min()
                    || x == maze_bbox.x_max()
                    || y == maze_bbox.y_min()
                    || y == maze_bbox.y_max()
                    || (x % 2 == 0 && y % 2 == 0)
                {
                    Square::Wall
                } else {
                    Square::Unknown { prio: 0 }
                }
            } else {
                Square::Unused
            }
        }))
    }
    fn get(&self, pos: Point2d) -> Square {
        self.0[pos]
    }
    fn set(&mut self, pos: Point2d, sq: Square) {
        self.0[pos] = sq;
    }

    fn set_orientation(&mut self, desired: Orientation, pos: Point2d) {
        let sq = self.get(pos);
        match sq {
            Square::Unknown { .. } => {
                let prio;
                if pos.x() % 2 == 0 && pos.y() % 2 != 0 {
                    // horizontal corridor, vertical wall
                    prio = if desired == Orientation::Horizontal {
                        10
                    } else {
                        -10
                    };
                } else if pos.x() % 2 != 0 && pos.y() % 2 == 0 {
                    // vertical corridor, horizontal wall
                    prio = if desired == Orientation::Vertical {
                        10
                    } else {
                        -10
                    };
                } else {
                    // always corridor at the end
                    prio = 0;
                }
                self.set(pos, Square::Unknown { prio });
            }
            _ => (),
        }
    }
    fn set_horizontal(&mut self, pos: Point2d) {
        self.set_orientation(Orientation::Horizontal, pos);
    }
    fn set_vertical(&mut self, pos: Point2d) {
        self.set_orientation(Orientation::Vertical, pos);
    }
    fn draw_horizontal_segment(&mut self, pos: Point2d, size: Vec2d) {
        for x in 0..size.x() {
            let xn = size.x() - 1 - x;
            self.set_horizontal(pos + v2d(x, 0));
            for y in 1..x.min(xn).min(size.y() / 2) + 1 {
                self.set_horizontal(pos + v2d(x, y));
                self.set_horizontal(pos + v2d(x, -y));
            }
        }
    }
    fn draw_vertical_segment(&mut self, pos: Point2d, size: Vec2d) {
        for y in 0..size.y() {
            let yn = size.y() - 1 - y;
            self.set_vertical(pos + v2d(0, y));
            for x in 1..y.min(yn).min(size.x() / 2) + 1 {
                self.set_vertical(pos + v2d(x, y));
                self.set_vertical(pos + v2d(-x, y));
            }
        }
    }
    fn draw_7_segments(&mut self, pos: Point2d, size: Vec2d, segments: u8) {
        let length = size.x();
        let width = size.y();
        let delta = length + 1;
        let hsize = v2d(length, width);
        let vsize = v2d(width, length);
        if (segments & (1 << 0)) != 0 {
            self.draw_horizontal_segment(pos + v2d(1, 0), hsize);
        }
        if (segments & (1 << 1)) != 0 {
            self.draw_vertical_segment(pos + v2d(delta, 1), vsize);
        }
        if (segments & (1 << 2)) != 0 {
            self.draw_vertical_segment(pos + v2d(delta, delta + 1), vsize);
        }
        if (segments & (1 << 3)) != 0 {
            self.draw_horizontal_segment(pos + v2d(1, 2 * delta), hsize);
        }
        if (segments & (1 << 4)) != 0 {
            self.draw_vertical_segment(pos + v2d(0, delta + 1), vsize);
        }
        if (segments & (1 << 5)) != 0 {
            self.draw_vertical_segment(pos + v2d(0, 1), vsize);
        }
        if (segments & (1 << 6)) != 0 {
            self.draw_horizontal_segment(pos + v2d(1, delta), hsize);
        }
    }
    fn draw_digit(&mut self, pos: Point2d, size: Vec2d, digit: u8) {
        let segment_table = vec![0x3f, 0x06, 0x5b, 0x4f, 0x66, 0x6d, 0x7d, 0x07, 0x7f, 0x6f];
        let segments = segment_table[digit as usize];
        self.draw_7_segments(pos, size, segments);
    }
}

#[derive(Debug)]
struct Move {
    from: Point2d,
    dir: Vec2d,
    prio: i32,
}

impl PartialEq for Move {
    fn eq(&self, other: &Move) -> bool {
        self.prio == other.prio
    }
}

impl Eq for Move {}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Move) -> Option<Ordering> {
        self.prio.partial_cmp(&other.prio)
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Move) -> Ordering {
        self.prio.cmp(&other.prio)
    }
}

fn add_move<R>(board: &Board, open: &mut BinaryHeap<Move>, rng: &mut R, from: Point2d, dir: Vec2d)
where
    R: Rng,
{
    if let Square::Unknown { prio } = board.get(from + dir) {
        open.push(Move {
            from,
            dir,
            prio: prio * 100 + rng.gen_range(0..1000),
        });
    }
}

const DEFAULT_ARG: isize = 16;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<i64>().unwrap();
    let y_size = args[2].parse::<i64>().unwrap();
    let arg = args[3].parse::<isize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let board_bbox = bb2d(0..x_size, 0..y_size);
    // round down to odd size for maze
    let maze_bbox = bb2d(0..(x_size - 1) / 2 * 2 + 1, 0..(y_size - 1) / 2 * 2 + 1);
    // pick odd position at or near the middle
    let maze_mid = maze_bbox.center();

    let mut board = Board::new(board_bbox, maze_bbox);
    let mut open = BinaryHeap::new();
    let mut last_drawn_minute = 99;
    loop {
        if open.is_empty() {
            // get time
            let dt = Local::now();
            let h = dt.hour();
            let m = dt.minute();

            if m != last_drawn_minute {
                last_drawn_minute = m;

                board = Board::new(board_bbox, maze_bbox);

                // draw time in prios
                let segment_size = v2d(11, 5);
                board.draw_digit(p2d(6, 8), segment_size, (h / 10) as u8);
                board.draw_digit(p2d(24, 8), segment_size, (h % 10) as u8);
                board.draw_digit(p2d(42, 8), segment_size, (m / 10) as u8);
                board.draw_digit(p2d(60, 8), segment_size, (m % 10) as u8);

                // start in the middle
                board.set(maze_mid, Square::Corridor);
                add_move(&board, &mut open, &mut rng, maze_mid, v2d(1, 0));
                add_move(&board, &mut open, &mut rng, maze_mid, v2d(0, 1));
                add_move(&board, &mut open, &mut rng, maze_mid, v2d(-1, 0));
                add_move(&board, &mut open, &mut rng, maze_mid, v2d(0, -1));
            }
        }

        // draw maze
        let work = arg.max(1);
        let mut count = 0;
        while !open.is_empty() && count < work {
            let Move {
                from: p0,
                dir: dir0,
                ..
            } = open.pop().unwrap();

            let p1 = p0 + dir0;
            let p2 = p1 + dir0;
            if board.get(p1).is_unknown() {
                board.set(p1, Square::Corridor);
                board.set(p2, Square::Corridor);

                for dir1 in Vec2d::unit_vecs_l1() {
                    let p3 = p2 + dir1;
                    let p4 = p3 + dir1;
                    if board.get(p3).is_unknown() {
                        if board.get(p4).is_unknown() {
                            add_move(&board, &mut open, &mut rng, p2, dir1);
                        } else {
                            board.set(p3, Square::Wall);
                        }
                    }
                }

                count += 1;
            }

            if open.is_empty() {
                board.set(maze_bbox.min() + v2d(1, 1), Square::Start);
                board.set(maze_bbox.max() - v2d(1, 1), Square::Finish);
            }
        }

        let mut buf = Vec::with_capacity(board_bbox.area() * 3);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

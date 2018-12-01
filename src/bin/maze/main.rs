use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use rand::thread_rng;
use rand::Rng;

use pixelfoo::color::Color;
use pixelfoo::p2;
use pixelfoo::point2d::Point2d;
use pixelfoo::v2;
use pixelfoo::vec2d::Vec2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Unknown,
    Corridor,
    Wall,
    Start,
    Finish,
}

struct Board(Vec<Vec<Square>>);

fn send<T: Write>(w: &mut T, board: &Board) -> std::io::Result<()> {
    for line in &board.0 {
        for square in line {
            let c = match square {
                Square::Unknown => Color::black(),
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

impl Board {
    fn new(board_size: Vec2d, maze_size: Vec2d) -> Board {
        Board(
            (0..board_size.y)
                .map(move |y| {
                    (0..board_size.x)
                        .map(|x| {
                            if x < maze_size.x && y < maze_size.y {
                                if x == 0
                                    || x == maze_size.x - 1
                                    || y == 0
                                    || y == maze_size.y - 1
                                    || (x % 2 == 0 && y % 2 == 0)
                                {
                                    Square::Wall
                                } else {
                                    Square::Unknown
                                }
                            } else {
                                Square::Unknown
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
    fn get(&self, pos: Point2d) -> Square {
        self.0[pos.y as usize][pos.x as usize]
    }
    fn set(&mut self, pos: Point2d, sq: Square) {
        self.0[pos.y as usize][pos.x as usize] = sq;
    }
}

struct Move {
    from: Point2d,
    dir: Vec2d,
    prio: i32,
}

fn add_move(open: &mut Vec<Move>, arg: isize, from: Point2d, dir: Vec2d) {
    open.push(Move { from, dir, prio: 0 });
    open.sort_unstable_by(|Move { prio: pa, .. }, Move { prio: pb, .. }| pa.cmp(pb));
}

const DEFAULT_ARG: isize = 16;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = args[3].parse::<isize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);
    let mut t_wait = 0.0; // s

    let board_size = v2!(x_size as i32, y_size as i32);
    // round down to odd size for maze
    let maze_size = v2!(
        (board_size.x - 1) / 2 * 2 + 1,
        (board_size.y - 1) / 2 * 2 + 1
    );
    // pick odd position at or near the middle
    let maze_mid = p2!((maze_size.x - 1) / 4 * 2 + 1, (maze_size.y - 1) / 4 * 2 + 1);

    let mut board = Board(Vec::new());
    let mut open = Vec::new();
    loop {
        if open.is_empty() {
            if t_wait > 0.0 {
                t_wait -= t_frame;
            } else {
                t_wait = 60.0; // s
                board = Board::new(board_size, maze_size);

                // start in the middle
                board.set(maze_mid, Square::Corridor);
                add_move(&mut open, arg, maze_mid, v2!(1, 0));
                add_move(&mut open, arg, maze_mid, v2!(0, 1));
                add_move(&mut open, arg, maze_mid, v2!(-1, 0));
                add_move(&mut open, arg, maze_mid, v2!(0, -1));
            }
        }

        // draw maze
        let work = arg.max(1);
        let mut count = 0;
        while !open.is_empty() && count < work {
            let mut start = 0;
            let mut limit = 0;
            let mut prio = open[limit].prio;
            loop {
                limit += 1;
                if limit >= open.len() {
                    break;
                }
                if open[limit].prio > prio {
                    if rng.gen::<f64>() >= 0.4 {
                        break;
                    }
                    start = limit;
                    prio = open[limit].prio;
                }
            }
            let r = rng.gen_range(start, limit);
            let Move {
                from: p0,
                dir: dir0,
                ..
            } = open.remove(r);

            let p1 = p0 + dir0;
            let p2 = p1 + dir0;
            if board.get(p1) == Square::Unknown {
                board.set(p1, Square::Corridor);
                board.set(p2, Square::Corridor);

                for dir1 in Vec2d::directions() {
                    let p3 = p2 + dir1;
                    let p4 = p3 + dir1;
                    if board.get(p3) == Square::Unknown {
                        if board.get(p4) == Square::Unknown {
                            add_move(&mut open, arg, p2, dir1);
                        } else {
                            board.set(p3, Square::Wall);
                        }
                    }
                }

                count += 1;
            }

            if open.is_empty() {
                board.set(p2!(1, 1), Square::Start);
                board.set(p2!(maze_size.x - 2, maze_size.y - 2), Square::Finish);
            }
        }

        let mut buf = Vec::with_capacity((board_size.x * board_size.y * 3) as usize);
        send(&mut buf, &board)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

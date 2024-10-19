use core::ops::Index;
use core::ops::IndexMut;

use std::convert::TryFrom;
use std::env::args;
use std::error;
use std::f64::consts::PI;
use std::io::stdout;
use std::io::Write;
use std::mem;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use rand::thread_rng;
use rand::Rng;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use pixelfoo::color::Color;

#[derive(Clone, Debug)]
struct RunningState {
    /// Time in frames since start of animation.
    t: i64,
    /// Number of frames per step in y direction.
    frames_per_step: i64,
    // Length of tail with constant brightness.
    tail_full: i64,
    // Length of tail with diminishing brightness.
    tail_fade: i64,
    /// Upper bound for the displayed y coordinate (exclusive).
    y_end: i64,
    /// Base brightness per pixel indexed by y.
    brightnesses: Vec<f64>,
}
impl RunningState {
    fn new(frames_per_step: i64, tail_full: i64, tail_fade: i64, y_end: i64) -> RunningState {
        RunningState {
            t: 0,
            frames_per_step,
            tail_full,
            tail_fade,
            y_end,
            brightnesses: Vec::new(),
        }
    }
    fn color(&self, y: i64) -> Color {
        let head_y = self.head_y();
        if y > head_y {
            Color::black()
        } else if y == head_y {
            Color::white()
        } else if y == head_y - 1 {
            let color = Color::white().interpolate(Color::green(), 0.3);
            let a = self.brightnesses[y as usize];
            Color::black().interpolate(color, a)
        } else if y >= head_y - self.tail_full {
            let a = self.brightnesses[y as usize];
            Color::black().interpolate(Color::green(), a)
        } else if y >= head_y - self.tail_full - self.tail_fade {
            let p = (head_y - self.tail_full - y + 1) as f64;
            let q = self.tail_fade as f64;
            let a = (1.0 - p / q) * self.brightnesses[y as usize];
            Color::black().interpolate(Color::green(), a)
        } else {
            Color::black()
        }
    }
    fn next_frame<R: Rng>(&mut self, rng: &mut R) {
        self.t += 1;
        let y = usize::try_from(self.head_y()).unwrap();
        if y >= self.brightnesses.len() {
            self.brightnesses.push(0.5 + 0.5 * rng.gen::<f64>());
        }
        if rng.gen::<f64>() < 0.2 {
            let y = rng.gen_range(0..self.brightnesses.len());
            self.brightnesses[y] = 0.5 + 0.5 * rng.gen::<f64>();
        }
    }
    fn head_y(&self) -> i64 {
        self.t / self.frames_per_step
    }
    fn is_finished(&self) -> bool {
        self.head_y() - self.tail_full - self.tail_full >= self.y_end
    }
}

#[derive(Clone, Debug, Default)]
enum AnimationState {
    #[default]
    Idle,
    Running(RunningState),
}
impl AnimationState {
    fn start(frames_per_step: i64, tail_full: i64, tail_fade: i64, y_end: i64) -> AnimationState {
        let state = RunningState::new(frames_per_step, tail_full, tail_fade, y_end);
        AnimationState::Running(state)
    }
    fn color(&self, y: i64) -> Color {
        match self {
            AnimationState::Idle => Color::black(),
            AnimationState::Running(state) => state.color(y),
        }
    }
    fn next_frame<R: Rng>(mut self, rng: &mut R) -> AnimationState {
        if let AnimationState::Running(state) = &mut self {
            state.next_frame(rng);
        }
        match self {
            AnimationState::Running(state) => {
                if state.is_finished() {
                    AnimationState::Idle
                } else {
                    AnimationState::Running(state)
                }
            }
            _ => self,
        }
    }
    fn is_idle(&self) -> bool {
        matches!(self, AnimationState::Idle)
    }
}

#[derive(Clone, Debug)]
struct Animation {
    state: AnimationState,
}
impl Animation {
    fn new() -> Animation {
        let state = AnimationState::Idle;
        Animation { state }
    }
    fn color(&self, y: i64) -> Color {
        self.state.color(y)
    }
    fn start(&mut self, frames_per_step: i64, tail_full: i64, tail_fade: i64, y_end: i64) {
        self.state = AnimationState::start(frames_per_step, tail_full, tail_fade, y_end);
    }
    fn next_frame<R: Rng>(&mut self, rng: &mut R) {
        let state = mem::take(&mut self.state);
        self.state = state.next_frame(rng);
    }
    fn is_idle(&self) -> bool {
        self.state.is_idle()
    }
}

#[derive(Clone, Debug)]
struct Frame(Array2d<i64, Color>);
impl Frame {
    pub fn new(bbox: BBox2d, color: Color) -> Frame {
        Frame(Array2d::new(bbox, color))
    }
    pub fn bbox(&self) -> BBox2d {
        self.0.bbox()
    }
}
impl Index<Point2d> for Frame {
    type Output = Color;
    fn index(&self, p: Point2d) -> &Color {
        &self.0[p]
    }
}
impl IndexMut<Point2d> for Frame {
    fn index_mut(&mut self, p: Point2d) -> &mut Color {
        &mut self.0[p]
    }
}

fn send<T: Write>(w: &mut T, frame: &Frame) -> std::io::Result<()> {
    for y in frame.bbox().y_range() {
        for x in frame.bbox().x_range() {
            let c = frame[p2d(x, y)];
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

const DEFAULT_ARG: u64 = 2;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<u64>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let mut rng = thread_rng();
    let bbox = bb2d(0..x_size as i64, 0..y_size as i64);

    let frame_duration = Duration::from_millis(50);

    let mut animations = (0..x_size)
        .map(|_| Animation::new())
        .collect::<Vec<Animation>>();

    let p_slow_animation = 0.1;

    let frames_per_step_normal = 4;
    let frames_per_step_slow = 8;

    let start_time = Instant::now();

    let mut frame_count = 0;
    let mut frame_time = start_time;
    let mut next_frame_time = frame_time + frame_duration;
    loop {
        let mut frame = Frame::new(bbox, Color::black());

        let a = PI * (frame_time - start_time).as_secs_f64() / 90.0;
        let p_new_animation = 0.03 * a.sin().powf(2.0);

        // Update the animation
        for animation in &mut animations {
            if animation.is_idle() {
                if rng.gen::<f64>() < p_new_animation {
                    let frames_per_step = {
                        if rng.gen::<f64>() < p_slow_animation {
                            frames_per_step_slow
                        } else {
                            frames_per_step_normal
                        }
                    };
                    if frame_count % frames_per_step == 0 {
                        let size = rng.gen_range(15..=30);
                        animation.start(frames_per_step, size, 15, i64::try_from(y_size)?);
                    }
                }
            } else {
                animation.next_frame(&mut rng);
            }
        }

        // Draw the state of the animations.
        for x in 0..x_size {
            for y in frame.bbox().y_range() {
                let p = p2d(i64::try_from(x)?, y);
                frame[p] = animations[x].color(y);
            }
        }

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;

        let sleep_duration = next_frame_time.saturating_duration_since(Instant::now());
        sleep(sleep_duration);

        stdout().write_all(&buf)?;
        stdout().flush()?;

        frame_time = next_frame_time;
        next_frame_time += frame_duration;
        frame_count += 1;
    }
}

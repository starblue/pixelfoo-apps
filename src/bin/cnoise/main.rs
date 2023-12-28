use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat_with;
use std::thread::sleep;
use std::time::Duration;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

use pixelfoo::color::Color;

type Frame = Vec<Vec<Color>>;

fn send<T: Write>(w: &mut T, f: &Frame) -> std::io::Result<()> {
    for l in f {
        for c in l {
            w.write_all(&c.rgb())?;
        }
    }
    w.flush()
}

#[derive(Clone, Debug)]
struct Palette(Vec<(Color, Color)>);
impl Palette {
    fn pick<R>(&self, rng: &mut R, p: f64) -> Color
    where
        R: Rng,
    {
        let &(c0, c1) = self.0.choose(rng).unwrap();

        if rng.gen::<f64>() < p {
            c0.interpolate(c1, rng.gen::<f64>().powf(2.0))
        } else {
            Color::black()
        }
    }
}

const DEFAULT_ARG: usize = 37;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = args[3].parse::<usize>().unwrap_or(DEFAULT_ARG);
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let mut rng = thread_rng();

    let palette_35 = Palette(vec![(Color::new(0, 138, 170), Color::new(0, 160, 95))]);
    let palette_37 = Palette(vec![
        (Color::new(153, 0, 0), Color::new(255, 0, 0)),
        (Color::new(0, 153, 0), Color::new(0, 255, 0)),
        (Color::new(0, 0, 153), Color::new(0, 0, 255)),
        (Color::new(153, 153, 0), Color::new(255, 255, 0)),
        (Color::new(0, 153, 153), Color::new(0, 255, 255)),
        (Color::new(153, 0, 153), Color::new(255, 0, 255)),
    ]);

    let palette = match arg {
        35 => palette_35,
        _ => palette_37,
    };

    let mut frame = repeat_with(|| {
        repeat_with(|| palette.pick(&mut rng, 0.5))
            .take(x_size)
            .collect::<Vec<_>>()
    })
    .take(y_size)
    .collect::<Vec<_>>();
    loop {
        let x = rng.gen_range(0..x_size);
        let y = rng.gen_range(0..y_size);
        let c = palette.pick(&mut rng, 0.5);
        frame[y][x] = c;

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

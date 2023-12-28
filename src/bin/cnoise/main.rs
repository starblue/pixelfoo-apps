use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat_with;
use std::thread::sleep;
use std::time::Duration;

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
struct Palette(Vec<(f64, Color, Color)>);
impl Palette {
    fn pick<R>(&self, rng: &mut R) -> Color
    where
        R: Rng,
    {
        let r = rng.gen::<f64>();
        let mut p_sum = 0.0;
        for &(p, c0, c1) in &self.0 {
            p_sum += p;
            if r < p_sum {
                return c0.interpolate(c1, rng.gen::<f64>().powf(2.0));
            }
        }
        Color::black()
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

    let palette_32 = Palette(vec![
        (0.06, Color::new(19, 4, 27), Color::new(19, 4, 27)),
        (0.06, Color::new(87, 23, 72), Color::new(87, 23, 72)),
        (0.06, Color::new(230, 32, 4), Color::new(230, 32, 4)),
        (0.06, Color::new(24, 0, 247), Color::new(24, 0, 247)),
        (0.06, Color::new(214, 121, 151), Color::new(214, 123, 151)),
    ]);
    let palette_33 = Palette(vec![
        (0.03, Color::new(0, 154, 147), Color::new(0, 154, 147)),
        (0.03, Color::new(117, 81, 156), Color::new(117, 81, 156)),
        (0.03, Color::new(255, 243, 116), Color::new(255, 243, 116)),
        (0.03, Color::new(236, 96, 144), Color::new(236, 96, 144)),
        (0.03, Color::new(0, 123, 196), Color::new(0, 123, 196)),
        (0.03, Color::new(162, 198, 23), Color::new(162, 198, 23)),
        (0.03, Color::new(231, 52, 76), Color::new(231, 52, 76)),
    ]);
    let palette_34 = Palette(vec![
        (0.07, Color::new(193, 193, 193), Color::new(193, 193, 193)),
        (0.07, Color::new(165, 36, 49), Color::new(165, 36, 49)),
        (0.07, Color::new(246, 151, 16), Color::new(246, 151, 16)),
    ]);
    let palette_35 = Palette(vec![(0.4, Color::new(0, 132, 176), Color::new(0, 163, 86))]);
    let palette_36 = Palette(vec![
        (0.1, Color::new(208, 208, 206), Color::new(208, 208, 206)),
        (0.05, Color::new(254, 80, 0), Color::new(254, 80, 0)),
        (0.05, Color::new(0, 187, 49), Color::new(0, 187, 49)),
    ]);
    let palette_37 = Palette(vec![
        (0.03, Color::new(255, 0, 0), Color::new(255, 0, 0)),
        (0.03, Color::new(0, 255, 0), Color::new(0, 255, 0)),
        (0.03, Color::new(0, 0, 255), Color::new(0, 0, 255)),
        (0.03, Color::new(255, 255, 0), Color::new(255, 255, 0)),
        (0.03, Color::new(0, 255, 255), Color::new(0, 255, 255)),
        (0.03, Color::new(255, 0, 255), Color::new(255, 0, 255)),
    ]);

    let palette = match arg {
        32 => palette_32,
        33 => palette_33,
        34 => palette_34,
        35 => palette_35,
        36 => palette_36,
        _ => palette_37,
    };

    let mut frame = repeat_with(|| {
        repeat_with(|| palette.pick(&mut rng))
            .take(x_size)
            .collect::<Vec<_>>()
    })
    .take(y_size)
    .collect::<Vec<_>>();
    loop {
        let x = rng.gen_range(0..x_size);
        let y = rng.gen_range(0..y_size);
        let c = palette.pick(&mut rng);
        frame[y][x] = c;

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

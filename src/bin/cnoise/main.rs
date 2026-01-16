use std::env::args;
use std::io::stdout;
use std::io::Write;
use std::iter::repeat_with;
use std::thread::sleep;
use std::time::Duration;

use rand::rng;
use rand::Rng;

use pixelfoo_apps::color::Color;

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
        let r = rng.random::<f64>();
        let mut p_sum = 0.0;
        for &(p, c0, c1) in &self.0 {
            p_sum += p;
            if r < p_sum {
                return c0.interpolate(c1, rng.random::<f64>().powf(2.0));
            }
        }
        Color::black()
    }
}

const DEFAULT_ARG: usize = 38;

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    eprintln!("executing {}", args[0]);

    let x_size = args[1].parse::<usize>().unwrap();
    let y_size = args[2].parse::<usize>().unwrap();
    let arg = if let Some(s) = args.get(3) {
        s.parse::<usize>().unwrap_or(DEFAULT_ARG)
    } else {
        DEFAULT_ARG
    };
    eprintln!("screen size {}x{}, arg {}", x_size, y_size, arg);

    let t_frame = 0.040; // s
    let delay = Duration::new(0, (1_000_000_000.0 * t_frame) as u32);

    let mut rng = rng();

    let palette_29 = Palette(vec![
        (0.04, Color::new(86, 197, 208), Color::new(86, 197, 208)),
        (0.04, Color::new(237, 26, 59), Color::new(237, 26, 59)),
        (0.04, Color::new(0, 178, 107), Color::new(0, 178, 107)),
        (0.04, Color::new(247, 148, 51), Color::new(247, 148, 51)),
        (0.04, Color::new(109, 104, 175), Color::new(109, 104, 175)),
    ]);
    let palette_30 = Palette(vec![
        (0.01, Color::new(0, 76, 85), Color::new(0, 76, 85)),
        (0.01, Color::new(0, 91, 102), Color::new(0, 91, 102)),
        (0.01, Color::new(0, 107, 119), Color::new(0, 107, 119)),
        (0.01, Color::new(0, 122, 136), Color::new(0, 122, 136)),
        (0.01, Color::new(0, 137, 153), Color::new(0, 137, 153)),
        (0.01, Color::new(0, 153, 171), Color::new(0, 153, 171)),
        (0.01, Color::new(0, 168, 188), Color::new(0, 168, 188)),
        (0.01, Color::new(0, 183, 205), Color::new(0, 183, 205)),
        (0.01, Color::new(0, 198, 222), Color::new(0, 198, 222)),
        (0.01, Color::new(0, 214, 239), Color::new(0, 214, 239)),
        (0.07, Color::new(0, 153, 171), Color::new(0, 0, 0)),
        (0.07, Color::new(0, 153, 171), Color::new(255, 255, 255)),
    ]);
    let palette_31 = Palette(vec![
        (0.06, Color::new(255, 90, 70), Color::new(255, 90, 70)),
        (0.06, Color::new(255, 130, 50), Color::new(255, 130, 50)),
        (0.06, Color::new(255, 160, 0), Color::new(255, 160, 0)),
        (0.06, Color::new(110, 80, 180), Color::new(110, 80, 180)),
        (0.06, Color::new(65, 20, 90), Color::new(65, 20, 90)),
    ]);
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
    let palette_38 = Palette(vec![
        (0.08, Color::new(255, 80, 83), Color::new(255, 80, 83)),
        (0.02, Color::new(254, 242, 255), Color::new(254, 242, 255)),
        (0.02, Color::new(178, 170, 255), Color::new(178, 170, 255)),
        (0.02, Color::new(106, 95, 219), Color::new(106, 95, 219)),
        (0.02, Color::new(38, 26, 102), Color::new(38, 26, 102)),
        (0.02, Color::new(41, 17, 76), Color::new(41, 17, 76)),
        (0.02, Color::new(25, 11, 47), Color::new(25, 11, 47)),
        (0.8, Color::new(15, 0, 10), Color::new(15, 0, 10)),
    ]);
    let palette_39 = Palette(vec![
        (0.84, Color::new(20, 20, 20), Color::new(20, 20, 20)),
        (0.03, Color::new(250, 245, 245), Color::new(250, 245, 245)),
        (0.04, Color::new(0, 255, 0), Color::new(0, 255, 0)),
        (0.06, Color::new(150, 115, 255), Color::new(150, 115, 255)),
        (0.01, Color::new(255, 55, 25), Color::new(255, 55, 25)),
        (0.02, Color::new(102, 242, 255), Color::new(102, 242, 255)),
    ]);

    let palette = match arg {
        29 => palette_29,
        30 => palette_30,
        31 => palette_31,
        32 => palette_32,
        33 => palette_33,
        34 => palette_34,
        35 => palette_35,
        36 => palette_36,
        37 => palette_37,
        38 => palette_38,
        _ => palette_39,
    };

    let mut frame = repeat_with(|| {
        repeat_with(|| palette.pick(&mut rng))
            .take(x_size)
            .collect::<Vec<_>>()
    })
    .take(y_size)
    .collect::<Vec<_>>();
    loop {
        let x = rng.random_range(0..x_size);
        let y = rng.random_range(0..y_size);
        let c = palette.pick(&mut rng);
        frame[y][x] = c;

        let mut buf = Vec::with_capacity(x_size * y_size * 3);
        send(&mut buf, &frame)?;
        stdout().write_all(&buf)?;
        stdout().flush()?;
        sleep(delay);
    }
}

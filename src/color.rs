#[derive(Clone, Copy, Debug)]
pub struct Color(u8, u8, u8);

#[allow(unused)]
impl Color {
    pub fn black() -> Color {
        Color(0, 0, 0)
    }
    pub fn white() -> Color {
        Color(255, 255, 255)
    }
    pub fn gray50() -> Color {
        Color(128, 128, 128)
    }
    pub fn red() -> Color {
        Color(255, 0, 0)
    }
    pub fn green() -> Color {
        Color(0, 255, 0)
    }
    pub fn blue() -> Color {
        Color(0, 0, 255)
    }
    pub fn yellow() -> Color {
        Color(255, 255, 0)
    }
    pub fn cyan() -> Color {
        Color(0, 255, 255)
    }
    pub fn magenta() -> Color {
        Color(255, 0, 255)
    }
    pub fn brown() -> Color {
        Color(210, 105, 30)
    }
    pub fn darkblue() -> Color {
        Color(0, 0, 127)
    }
    pub fn darkbrown() -> Color {
        Color(139, 69, 19)
    }
    pub fn complement(&self) -> Color {
        Color(255 - self.0, 255 - self.1, 255 - self.2)
    }

    pub fn rgb(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }

    pub fn interpolate(self, c: Color, a: f64) -> Color {
        Color(
            interpolate_u8(self.0, c.0, a),
            interpolate_u8(self.1, c.1, a),
            interpolate_u8(self.2, c.2, a),
        )
    }
}

fn interpolate_u8(b0: u8, b1: u8, a: f64) -> u8 {
    let b0 = b0 as f64;
    let b1 = b1 as f64;
    ((1.0 - a) * b0 + a * b1) as u8
}

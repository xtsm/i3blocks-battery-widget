use std::fmt::{self, Display};

fn interpolate(a: u8, b: u8, x: f64) -> u8 {
    let a = a as f64;
    let b = b as f64;
    (a + (b - a) * x).round() as u8
}

#[derive(Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

struct GradientPoint {
    x: f64,
    color: RGB,
}

impl GradientPoint {
    fn interpolate(&self, oth: &GradientPoint, x: f64) -> RGB {
        let x = (x - self.x) / (oth.x - self.x);
        RGB {
            r: interpolate(self.color.r, oth.color.r, x),
            g: interpolate(self.color.g, oth.color.g, x),
            b: interpolate(self.color.b, oth.color.b, x),
        }
    }
}

pub struct Gradient {
    points: Vec<GradientPoint>,
}

impl Gradient {
    pub fn new() -> Gradient {
        Gradient {
            points: vec![
                GradientPoint {
                    x: 0.0,
                    color: RGB {
                        r: 0x66,
                        g: 0x00,
                        b: 0x00,
                    },
                },
                GradientPoint {
                    x: 0.2,
                    color: RGB {
                        r: 0xff,
                        g: 0x66,
                        b: 0x33,
                    },
                },
                GradientPoint {
                    x: 0.6,
                    color: RGB {
                        r: 0xff,
                        g: 0xff,
                        b: 0x66,
                    },
                },
                GradientPoint {
                    x: 1.0,
                    color: RGB {
                        r: 0x33,
                        g: 0xff,
                        b: 0x66,
                    },
                },
            ],
        }
    }

    pub fn get(&self, x: f64) -> RGB {
        let first = &self.points.first().unwrap();
        if x < first.x {
            return first.color;
        }
        for i in 1..self.points.len() {
            if x < self.points[i].x {
                return self.points[i - 1].interpolate(&self.points[i], x);
            }
        }
        self.points.last().unwrap().color
    }
}

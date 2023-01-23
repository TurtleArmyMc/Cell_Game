pub trait Color: ToString + Copy {}

#[derive(Clone, Copy)]
pub struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl RGBA {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl ToString for RGBA {
    fn to_string(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

impl Color for RGBA {}

#[derive(Clone, Copy)]
pub struct HSL {
    hue: u8,
    saturation: u8,
    lightness: u8,
}

impl HSL {
    pub fn new(hue: u8, saturation: u8, lightness: u8) -> Self {
        Self {
            hue,
            saturation,
            lightness,
        }
    }
}

impl ToString for HSL {
    fn to_string(&self) -> String {
        format!(
            "hsl({},{}%,{}%)",
            self.hue,
            (self.saturation as f64 / u8::MAX as f64) * 100.0,
            (self.lightness as f64 / u8::MAX as f64) * 100.0,
        )
    }
}

impl Color for HSL {}

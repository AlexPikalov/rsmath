use std::ops::{Add, Mul};

pub struct Complex {
    pub im: f64,
    pub re: f64,
}

impl Complex {
    pub fn exp(exp: Complex) -> Complex {
        return Complex {
            im: exp.re.exp() * exp.im.sin(),
            re: exp.re.exp() * exp.im.cos(),
        };
    }

    pub fn conj(&self) -> Complex {
        return Complex {
            im: -self.im,
            re: self.re,
        };
    }

    pub fn norm(&self) -> f64 {
        return (self.re.powi(2) + self.im.powi(2)).sqrt();
    }

    pub fn arg(&self) -> f64 {
        return (self.im / self.re).atan();
    }

    pub fn pow(self, p: Complex) -> Complex {
        let norm = self.norm();
        let arg = self.arg();
        return Complex {
            im: norm * (-arg * p.im).exp() * (arg * p.re).sin(),
            re: norm * (-arg * p.im).exp() * (arg * p.re).cos(),
        };
    }

    // Main branch of complex logarithm
    pub fn ln_m(&self) -> Complex {
        let norm = self.norm();
        let arg = self.arg();
        return Complex {
            im: arg,
            re: norm.ln(),
        };
    }

    pub fn sin(&self) -> Complex {
        return Complex {
            im: -self.re.sin() * self.im.sinh(),
            re: self.re.cos() * self.im.cosh(),
        };
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        return Complex {
            im: self.im + rhs.im,
            re: self.re + rhs.re,
        };
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        return Complex {
            im: self.re * rhs.im + self.im * rhs.re,
            re: self.re * rhs.re - self.im * rhs.im,
        };
    }
}

impl From<f64> for Complex {
    fn from(n: f64) -> Complex {
        return Complex { im: 0.0, re: n };
    }
}

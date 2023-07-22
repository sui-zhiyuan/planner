#[cfg(test)]
mod test;

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul},
};

/// unsigned Fraction
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Fraction(u32, u32);

impl Fraction {
    pub fn new(amount: u32, unit: u32) -> Self {
        if unit == 0 {
            panic!("unit can not be 0");
        }
        let g = gcd(amount, unit);
        Fraction(amount / g, unit / g)
    }

    pub fn cell(&self) -> Self {
        let m = self.0 % self.1;
        let m = if m > 0 { 1 } else { 0 };
        Fraction(self.0 / self.1 + m, 1)
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        let unit = lcm(self.1, rhs.1);
        let amount = self.0 * (unit / self.1) + rhs.0 * (unit / rhs.1);
        Fraction::new(amount, unit)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl From<u32> for Fraction {
    fn from(value: u32) -> Self {
        Fraction(value, 1)
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Fraction(amount, unit) = self;
        if self.1 == 1 {
            write!(f, "{amount}")?;
            return Ok(());
        }
        write!(f, "{amount}/{unit}")?;
        Ok(())
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amount = self.0 as f64;
        let unit = self.1 as f64;
        let v = amount / unit;
        Display::fmt(&v, f)
    }
}

fn gcd(l: u32, r: u32) -> u32 {
    let (mut l, mut r) = (l, r);
    if l < r {
        (l, r) = (r, l);
    };

    loop {
        if r == 0 {
            return l;
        }
        (l, r) = (r, l % r)
    }
}

fn lcm(l: u32, r: u32) -> u32 {
    if l == 0 && r == 0 {
        panic!("no lcm for 0 and 0")
    }
    let g = gcd(l, r);
    g * (l / g) * (r / g)
}

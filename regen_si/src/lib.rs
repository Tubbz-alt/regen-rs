use std::ops::{Add, Mul};
use std::fmt::{Display, Formatter, Error};

#[derive(Display)]
enum BaseUnit {
    Second = 1,
    Metre = 2,
    Kilogram = 3,
    Ampere = 4,
    Kelvin = 5,
    Mole = 6,
    Candela = 7,
}

struct Unit([i32; 7]);

impl Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self::Output {
        let mut powers = [0; 7];
        for i in 1..=7 {
            powers[i] = self.0[i] + rhs.0[i]
        }
        Unit(powers)
    }
}

impl Mul for BaseUnit {
    type Output = Unit;

    fn mul(self, pow: i32) -> Self::Output {
        let mut powers = [0; 7];
        powers[self] = rhs;
        Unit(powers)
    }
}

impl Display for BaseUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Unit {
    fn name(&self) -> &str {
        match Self {}
    }

    fn base_quantity(&self) -> &str {
        match Self {}
    }

    fn symbol(&self) -> &str {
        match Self {}
    }
}

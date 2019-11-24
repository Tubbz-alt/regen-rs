use std::ops::{Add, Mul, Sub, Div};
use std::fmt::{Display, Formatter, Error};

//#[derive(Display)]
enum BaseUnit {
    Second = 1,
    Metre = 2,
    Kilogram = 3,
    Ampere = 4,
    Kelvin = 5,
    Mole = 6,
    Candela = 7,
}

pub struct QuantityDimension {
    pub time_exponent: i32,
    pub length_exponent: i32,
    pub mass_exponent: i32,
    pub electric_current_exponent: i32,
    pub thermodynamic_temperature_exponent: i32,
    pub amount_of_substance_exponent: i32,
    pub luminous_intensity_exponent: i32,
    pub dimensionless_exponent: i32,
//    pub dimensionless: Option<Dimensionless>
}

pub struct QuantityKind {
    pub dimension: QuantityDimension
}

enum Dimensionless {
    Angle,
    SolidAngle,
    Information,
//    ConstituentConcentration // found this in uom, not sure how useful/common it is
}

pub struct Unit<Number: Add + Sub + Mul + Div + PartialOrd> {
    pub kind: QuantityKind,
    pub conversion_multipler: Number,
    pub conversion_offset: Number,

}

pub struct QuantityValue<Number: Add + Sub + Mul + Div + PartialOrd> {
    pub number: Number,
    pub unit: Unit<Number>,
}

pub struct MeasuredQuantityValue<Number: Add + Sub + Mul + Div + PartialOrd> {
    pub number: Number,
    pub unit: Unit<Number>,
    pub uncertainty: Number,
}


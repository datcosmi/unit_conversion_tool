#[derive(Copy, Clone, PartialEq)]
pub enum Unit {
    Meters,
    Kilometers,
    Centimeters,
    Inches,
    Feet,
}

impl Unit {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Meters => "Meters",
            Self::Kilometers => "Kilometers",
            Self::Centimeters => "Centimeters",
            Self::Inches => "Inches",
            Self::Feet => "Feet",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Meters => "m",
            Self::Kilometers => "km",
            Self::Centimeters => "cm",
            Self::Inches => "in",
            Self::Feet => "ft",
        }
    }
}

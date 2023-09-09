// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

static EARTH_YEAR_IN_SECONDS: f64 = 31557600_f64;

#[derive(Debug)]
pub struct Duration {
    earth_age: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let earth_age = s as f64 / EARTH_YEAR_IN_SECONDS;
        Self { earth_age }
    }
}

pub trait Planet {
    fn orbital_years() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_age / Self::orbital_years()
    }
}

macro_rules! planet {
    ($planet:ident, $orbital_years:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn orbital_years() -> f64 {
                $orbital_years
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

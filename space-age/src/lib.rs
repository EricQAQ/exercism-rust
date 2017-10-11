// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#![allow(unused_variables)]

pub struct Duration {
    duration: f64,
}

impl Duration {
    pub fn new(duration: f64) -> Self {
        Duration { duration: duration }
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration::new(s as f64)
    }
}

impl From<f64> for Duration {
    fn from(s: f64) -> Self {
        Duration::new(s)
    }
}

pub trait Planet {
    fn orbital_period_seconds() -> Duration;

    fn years_during(d: &Duration) -> f64 {
        d.duration / Self::orbital_period_seconds().duration
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 0.2_408_467)
    }
}
impl Planet for Venus {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 0.61_519_726)
    }
}
impl Planet for Earth {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600)
    }
}
impl Planet for Mars {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 1.8_808_158)
    }
}
impl Planet for Jupiter {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 11.862_615)
    }
}
impl Planet for Saturn {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 29.447_498)
    }
}
impl Planet for Uranus {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 84.016_846)
    }
}
impl Planet for Neptune {
    fn orbital_period_seconds() -> Duration {
        Duration::from(31_557_600_f64 * 164.79_132)
    }
}

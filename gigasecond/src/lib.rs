extern crate chrono;
use chrono::*;

static giga_second: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let duration = Duration::seconds(giga_second);
    start + duration
}

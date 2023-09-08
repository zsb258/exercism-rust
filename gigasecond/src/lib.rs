use time::PrimitiveDateTime as DateTime;

static GIGASECONDS: time::Duration = time::Duration::seconds(1e9 as i64);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + GIGASECONDS
}

extern crate chrono;

use chrono::prelude::*;

static FMT_DATETAG: &'static str = "%Y%m%d%H%M%S";
static FMT_DAYOFTHEYEAR: &'static str = "%j";
static FMT_WEEKOFTHEYEAR: &'static str = "%W";

pub fn datetag_utc() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.format(FMT_DATETAG).to_string()
}

pub fn datetag_local() -> String {
    let local: DateTime<Local> = Local::now();
    local.format(FMT_DATETAG).to_string()
}

pub fn datetag() -> String {
    datetag_local()
}

// %j: Day of the year (001--366), zero-padded to 3 digits.
pub fn day_of_the_year() -> String {
    let local: DateTime<Local> = Local::now();
    local.format(FMT_DAYOFTHEYEAR).to_string()
}

// %W: Same to %U, but week 1 starts with the first Monday in that year instead.
pub fn week_of_the_year() -> String {
    let local: DateTime<Local> = Local::now();
    local.format(FMT_WEEKOFTHEYEAR).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

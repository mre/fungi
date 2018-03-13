extern crate chrono;

use chrono::prelude::*;

static FMT_DATETAG: &'static str = "%Y%m%d%H%M%S";
static FMT_DAYOFTHEYEAR: &'static str = "%j";

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

pub fn day_of_the_year() -> String {
    let local: DateTime<Local> = Local::now();
     local.format(FMT_DAYOFTHEYEAR).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

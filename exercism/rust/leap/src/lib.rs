pub fn is_leap_year(year: u64) -> bool {
    // let divisible_by = |divisor: i32| -> bool {(year % divisor) == 0};
    // `year` is divisible by 400, or 4 but not 100.
    // divisible_by(400) || (divisible_by(4) && !(divisible_by(100)))
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

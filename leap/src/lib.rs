pub fn is_leap_year(year: u64) -> bool {
    let v4 = year % 4 == 0;
    let v100 = year % 100 == 0;
    let v400 = year % 400 == 0;

    v4 && (!v100 || v400)
}

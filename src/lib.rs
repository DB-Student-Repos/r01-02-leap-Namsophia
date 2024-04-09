pub fn is_leap_year(year: u64) -> bool {
   // unimplemented!("true if {year} is a leap year");
    if year % 100 == 0 && year % 400 == 0 {
        return true;
    }
    if year % 10 == 0 {
        return false;
    }
    if year % 4 == 0 {
        return true;
    }
    return false;

}

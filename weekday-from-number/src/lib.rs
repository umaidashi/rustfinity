pub fn weekday_from_number(day: u8) -> &'static str {
    // 'static is a special lifetime that means the string is stored in the binary
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day number",
    }
}

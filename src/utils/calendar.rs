// Calendar utilities for date manipulation without external dependencies.
//
// This module provides lightweight date calculations for the DatePicker widget.
// Supports years 1970-2099 with proper leap year handling.

/// Represents which day starts the week
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum WeekStart {
    #[default]
    Monday,
    Sunday,
}

/// Simple date structure (year, month 1-12, day 1-31)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32, // 1-12
    pub day: u32,   // 1-31
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Format as dd.mm.yyyy
    pub fn format_dmy(&self) -> String {
        format!("{:02}.{:02}.{:04}", self.day, self.month, self.year)
    }

    /// Format as yyyy-mm-dd (ISO)
    pub fn format_iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Get the current date from system time (local timezone)
pub fn today() -> SimpleDate {
    // Try to get local time first
    if let Ok(local) = time::OffsetDateTime::now_local() {
        return SimpleDate {
            year: local.year(),
            month: local.month() as u32,
            day: local.day() as u32,
        };
    }

    // Fallback to UTC if local time fails
    let utc = time::OffsetDateTime::now_utc();
    SimpleDate {
        year: utc.year(),
        month: utc.month() as u32,
        day: utc.day() as u32,
    }
}

/// Convert Unix timestamp (seconds since 1970-01-01) to SimpleDate
fn unix_to_date(timestamp: i64) -> SimpleDate {
    // Days since epoch
    let mut days = (timestamp / 86400) as i32;

    // Start from 1970
    let mut year = 1970;

    // Find the year
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    // Find the month
    let mut month = 1u32;
    loop {
        let days_in_month = days_in_month(year, month);
        if days < days_in_month as i32 {
            break;
        }
        days -= days_in_month as i32;
        month += 1;
    }

    SimpleDate {
        year,
        month,
        day: (days + 1) as u32,
    }
}

/// Check if a year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get the number of days in a month (1-12)
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,  // January
        2 => if is_leap_year(year) { 29 } else { 28 }, // February
        3 => 31,  // March
        4 => 30,  // April
        5 => 31,  // May
        6 => 30,  // June
        7 => 31,  // July
        8 => 31,  // August
        9 => 30,  // September
        10 => 31, // October
        11 => 30, // November
        12 => 31, // December
        _ => 30,  // Fallback (should not happen)
    }
}

/// Get the day of week for a date (0 = Monday, 6 = Sunday)
/// Uses Zeller's congruence algorithm adapted for Monday = 0
pub fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
    // Adjust for Zeller's formula (March = 3, ..., February = 14)
    let (y, m) = if month < 3 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    let q = day as i32;
    let m = m as i32;
    let k = y % 100;
    let j = y / 100;

    // Zeller's congruence gives 0 = Saturday, 1 = Sunday, 2 = Monday, etc.
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;

    // Convert to 0 = Monday, 6 = Sunday
    let h = ((h + 5) % 7 + 7) % 7;
    h as u32
}

/// Get the day of week for the first day of a month (0 = Monday, 6 = Sunday)
pub fn first_day_of_month(year: i32, month: u32) -> u32 {
    day_of_week(year, month, 1)
}

/// Generate calendar grid data for a month
/// Returns a vector of (day_number, row, col) for each day
/// row and col are 0-indexed, suitable for grid layout
pub fn calendar_grid(year: i32, month: u32, week_start: WeekStart) -> Vec<(u32, u32, u32)> {
    let days = days_in_month(year, month);
    let first_dow = first_day_of_month(year, month);

    // Adjust for week start
    let offset = match week_start {
        WeekStart::Monday => first_dow,
        WeekStart::Sunday => (first_dow + 1) % 7,
    };

    let mut result = Vec::with_capacity(days as usize);
    for day in 1..=days {
        let position = offset + day - 1;
        let row = position / 7;
        let col = position % 7;
        result.push((day, row, col));
    }
    result
}

/// Get short day names based on week start
pub fn day_names_short(week_start: WeekStart) -> [&'static str; 7] {
    match week_start {
        WeekStart::Monday => ["M", "T", "W", "T", "F", "S", "S"],
        WeekStart::Sunday => ["S", "M", "T", "W", "T", "F", "S"],
    }
}

/// Get the number of rows needed for a month's calendar grid
pub fn calendar_rows(year: i32, month: u32, week_start: WeekStart) -> u32 {
    let days = days_in_month(year, month);
    let first_dow = first_day_of_month(year, month);

    let offset = match week_start {
        WeekStart::Monday => first_dow,
        WeekStart::Sunday => (first_dow + 1) % 7,
    };

    // Last day's position
    let last_position = offset + days - 1;
    (last_position / 7) + 1
}

/// Month names (full)
pub const MONTH_NAMES: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

/// Month names (short)
pub const MONTH_NAMES_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 2), 29); // Leap year
        assert_eq!(days_in_month(2023, 2), 28); // Non-leap year
        assert_eq!(days_in_month(2024, 1), 31);
        assert_eq!(days_in_month(2024, 4), 30);
    }

    #[test]
    fn test_day_of_week() {
        // 2024-01-01 is Monday
        assert_eq!(day_of_week(2024, 1, 1), 0);
        // 2024-01-07 is Sunday
        assert_eq!(day_of_week(2024, 1, 7), 6);
    }

    #[test]
    fn test_calendar_grid() {
        let grid = calendar_grid(2024, 1, WeekStart::Monday);
        // January 2024 starts on Monday (col 0)
        assert_eq!(grid[0], (1, 0, 0)); // Day 1 at row 0, col 0
    }
}

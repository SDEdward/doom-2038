use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The Unix timestamp for the Year 2038 problem:
/// 2038-01-19 03:14:07 UTC. This timestamp represents
/// the maximum value for a 32-bit signed integer.
///
/// # Examples
///
/// ```
/// let u32_limit = DOOM_TS;  
/// assert_eq!(u32_limit, 2_147_483_647);
/// ```
pub const DOOM_TS: u64 = 2_147_483_647;

/// ammount of seconds in a minute
const SECONDS_IN_MINUTE: u64 = 60;
/// ammount of seconds in a hour
const SECONDS_IN_HOUR: u64 = 3600;
/// ammount of seconds in a day
const SECONDS_IN_DAY: u64 = 86_400; // 24 * 60 * 60
/// ammount of seconds in a year (why ts scarily small)
const SECONDS_IN_YEAR: u64 = 31_536_000; // 365 * 24 * 60 * 60

/// Calculates the time left until the Year 2038 problem occurs,
/// returning it as a `Duration`.
///
/// This function computes the difference between the current time
/// and the DOOM_TS constant, which marks the limit of a 32-bit
/// unsigned integer. If the current time is past the doom timestamp,
/// this will return a `Duration` of zero.
///
/// # Returns
///
/// A `Duration` representing the time remaining until the Year 2038 problem.
///
/// # Examples
///
/// ```
/// let duration_left = time_left();
/// println!("Time left until 2038 problem: {:?}", duration_left);
/// ```
pub fn time_left() -> Duration {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Duration::from_secs(DOOM_TS.saturating_sub(now.as_secs()))
}

/// Displays the time left until whatever in a
/// human-readable format.
///
/// # Parameters
///
/// - `duration`: A `Duration` indicating the time left.
///
/// # Examples
///
/// ```
/// let remaining_time = Duration::new(5000, 0); // Example duration
/// print_time_left(remaining_time);
/// ```
pub fn print_time_left(duration: Duration) {
    let total_seconds = duration.as_secs();

    if total_seconds >= SECONDS_IN_YEAR {
        let years = total_seconds / SECONDS_IN_YEAR;
        let remaining_seconds = total_seconds % SECONDS_IN_YEAR;

        let days = remaining_seconds / SECONDS_IN_DAY;
        let remaining_seconds = remaining_seconds % SECONDS_IN_DAY;

        let hours = remaining_seconds / SECONDS_IN_HOUR;
        let remaining_seconds = remaining_seconds % SECONDS_IN_HOUR;

        let minutes = remaining_seconds / SECONDS_IN_MINUTE;
        let seconds = remaining_seconds % SECONDS_IN_MINUTE;

        println!(
            "{} years, {} days, {} hours, {} minutes, {} seconds",
            years, days, hours, minutes, seconds
        );
    } else if total_seconds >= SECONDS_IN_DAY {
        let days = total_seconds / SECONDS_IN_DAY;
        let remaining_seconds = total_seconds % SECONDS_IN_DAY;

        let hours = remaining_seconds / SECONDS_IN_HOUR;
        let remaining_seconds = remaining_seconds % SECONDS_IN_HOUR;

        let minutes = remaining_seconds / SECONDS_IN_MINUTE;
        let seconds = remaining_seconds % SECONDS_IN_MINUTE;

        println!(
            "{} days, {} hours, {} minutes, {} seconds",
            days, hours, minutes, seconds
        );
    } else if total_seconds >= SECONDS_IN_HOUR {
        let hours = total_seconds / SECONDS_IN_HOUR;
        let remaining_seconds = total_seconds % SECONDS_IN_HOUR;

        let minutes = remaining_seconds / SECONDS_IN_MINUTE;
        let seconds = remaining_seconds % SECONDS_IN_MINUTE;

        println!("{} hours, {} minutes, {} seconds", hours, minutes, seconds);
    } else if total_seconds >= SECONDS_IN_MINUTE {
        let minutes = total_seconds / SECONDS_IN_MINUTE;
        let seconds = total_seconds % SECONDS_IN_MINUTE;

        println!("{} minutes, {} seconds", minutes, seconds);
    } else {
        println!("{} seconds", total_seconds);
    }
}

/// Runs the doom countdown either continuously or just once,
/// based on the `countdown` flag.
///
/// If `countdown` is set to true, the function enters a loop
/// that refreshes and prints the time left every second.
/// If false, it simply prints the time left once.
///
/// # Parameters
///
/// - `countdown`: A boolean that determines whether to run the countdown continuously or just once. (true = continously)
///
/// # Examples
///
/// ```
/// // To run the countdown continuously:
/// doom(true);
///
/// // To print the time left once:
/// doom(false);
/// ```
pub fn doom(countdown: bool) {
    if countdown {
        loop {
            print_time_left(time_left());
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        print_time_left(time_left());
    }
}

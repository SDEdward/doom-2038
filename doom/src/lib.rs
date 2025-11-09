use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The Unix timestamp for the Year 2038 problem: 2038-01-19 03:14:07 UTC
pub const DOOM_TS: u64 = 2_147_483_647;

/// Returns the time left until the Year 2038 problem as a `Duration`.
pub fn time_left() -> Duration {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Duration::from_secs(DOOM_TS.saturating_sub(now.as_secs()))
}

/// Prints the time left in a human-readable format.
fn print_time_left(duration: Duration) {
    println!(
        "{} days, {} hours, {} minutes, {} seconds",
        duration.as_secs() / 86400,
        (duration.as_secs() % 86400) / 3600,
        (duration.as_secs() % 3600) / 60,
        duration.as_secs() % 60
    );
}

/// Runs the doom countdown once or continuously depending on the `countdown` flag.
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

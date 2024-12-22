use chrono::{DateTime, Duration};

pub fn convert_to_datetime(timestamp: i64) -> String {
    let utc_tz = DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp");
    let jakarta_tz = utc_tz + Duration::hours(7);

    jakarta_tz.format("%Y-%m-%d %H:%M:%S").to_string()
}
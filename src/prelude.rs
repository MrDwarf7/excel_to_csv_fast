#[allow(unused_imports)]
use chrono::TimeZone;

use std::sync::OnceLock;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static CHARS: OnceLock<Vec<char>> = OnceLock::new();

pub fn get_chars() -> &'static Vec<char> {
    CHARS.get_or_init(|| vec!['\r', '\n', '_'])
}

static UTF8_CODES: OnceLock<Vec<u8>> = OnceLock::new();

pub fn get_utf8_codes() -> &'static Vec<u8> {
    UTF8_CODES.get_or_init(|| get_chars().iter().map(|c| *c as u8).collect())
}

static CHUNK: OnceLock<String> = OnceLock::new();

pub fn get_chunk() -> &'static String {
    CHUNK.get_or_init(|| "x000D".to_string())
}

// Wrapper struct
// #[allow(dead_code)]
// pub struct W<T>(pub T);

// impl TryFrom<W<f64>> for chrono::FixedOffset {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: W<f64>) -> std::result::Result<Self, Self::Error> {}
// }

// impl TryFrom<W<&str>> for chrono::DateTime<chrono::FixedOffset> {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: W<&str>) -> Result<Self> {
//         let local_dt =
//             chrono::DateTime::<chrono::FixedOffset>::parse_from_str(value.0, "%dd-%M-%YYYY %H:%M")?;
//         Ok(local_dt)
//     }
// }
//
// impl TryFrom<W<&str>> for chrono::DateTime<chrono::Utc> {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: W<&str>) -> Result<Self> {
//         let date_time = chrono::DateTime::parse_from_str(value.0, "%dd-%M-%YYYY %H:%M")?;
//         Ok(date_time.with_timezone(&chrono::Utc))
//     }
// }
//
// impl TryFrom<W<f64>> for chrono::DateTime<chrono::Utc> {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: W<f64>) -> Result<Self> {
//         let date_time = chrono::Utc.timestamp(value.0 as i64, 0);
//         Ok(date_time)
//     }
// }
//
// impl TryFrom<W<f64>> for chrono::DateTime<chrono::FixedOffset> {
//     type Error = Box<dyn std::error::Error>;
//     fn try_from(value: W<f64>) -> Result<Self> {
//         let local_tz = chrono::FixedOffset::
//         Ok(date_time)
//     }
// }

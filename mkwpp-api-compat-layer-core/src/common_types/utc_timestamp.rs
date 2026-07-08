use mkwpp_api_compat_layer_macros::FromIntoInner;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, FromIntoInner, Default)]
#[cfg_attr(feature = "typescript-wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct UtcTimestamp(f64);

#[cfg(feature = "chrono-conversion")]
mod chrono_conversions {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

    use crate::common_types::utc_timestamp::UtcTimestamp;

    // Chrono -> UtcTimestamp
    impl<Tz: TimeZone> From<DateTime<Tz>> for UtcTimestamp {
        fn from(value: DateTime<Tz>) -> Self {
            Self::from(value.timestamp_millis() as f64)
        }
    }

    impl From<NaiveDate> for UtcTimestamp {
        fn from(value: NaiveDate) -> Self {
            Self::from(unsafe { value.and_hms_opt(0, 0, 0).unwrap_unchecked() })
        }
    }

    impl From<NaiveDateTime> for UtcTimestamp {
        fn from(value: NaiveDateTime) -> Self {
            Self::from(value.and_utc())
        }
    }

    // UtcTimestamp -> Chrono
    impl From<UtcTimestamp> for DateTime<Utc> {
        fn from(value: UtcTimestamp) -> Self {
            unsafe { DateTime::from_timestamp_millis(value.0 as i64).unwrap_unchecked() }
        }
    }

    impl From<UtcTimestamp> for NaiveDateTime {
        fn from(value: UtcTimestamp) -> Self {
            DateTime::<Utc>::from(value).naive_utc()
        }
    }

    impl From<UtcTimestamp> for NaiveDate {
        fn from(value: UtcTimestamp) -> Self {
            DateTime::<Utc>::from(value).date_naive()
        }
    }
}

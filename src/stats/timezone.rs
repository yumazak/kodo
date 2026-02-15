use chrono::{DateTime, Local, Utc};
use chrono_tz::Tz;

#[derive(Debug, Clone)]
pub enum TimeZoneMode {
    Local,
    Utc,
    Named(Tz),
}

impl TimeZoneMode {
    pub fn parse(input: &str) -> Result<Self, String> {
        let normalized = input.trim();
        if normalized.eq_ignore_ascii_case("local") {
            return Ok(Self::Local);
        }
        if normalized.eq_ignore_ascii_case("utc") {
            return Ok(Self::Utc);
        }

        normalized
            .parse::<Tz>()
            .map(Self::Named)
            .map_err(|_| format!("invalid timezone: {input}. Use local, utc, or an IANA name like Asia/Tokyo"))
    }

    pub fn date_naive(&self, utc: DateTime<Utc>) -> chrono::NaiveDate {
        match self {
            Self::Local => utc.with_timezone(&Local).date_naive(),
            Self::Utc => utc.date_naive(),
            Self::Named(tz) => utc.with_timezone(tz).date_naive(),
        }
    }

    pub fn datetime(&self, utc: DateTime<Utc>) -> DateTime<chrono::FixedOffset> {
        match self {
            Self::Local => utc.with_timezone(&Local).fixed_offset(),
            Self::Utc => utc.fixed_offset(),
            Self::Named(tz) => utc.with_timezone(tz).fixed_offset(),
        }
    }

    pub fn now_date_naive(&self) -> chrono::NaiveDate {
        match self {
            Self::Local => Local::now().date_naive(),
            Self::Utc => Utc::now().date_naive(),
            Self::Named(tz) => Utc::now().with_timezone(tz).date_naive(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn parses_named_timezone() {
        let tz = TimeZoneMode::parse("America/New_York").expect("valid timezone");
        assert!(matches!(tz, TimeZoneMode::Named(_)));
    }

    #[test]
    fn handles_dst_offsets_for_named_timezone() {
        let tz = TimeZoneMode::parse("America/New_York").expect("valid timezone");

        let winter = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        let summer = Utc.with_ymd_and_hms(2024, 7, 15, 12, 0, 0).unwrap();

        let winter_offset = tz.datetime(winter).offset().local_minus_utc();
        let summer_offset = tz.datetime(summer).offset().local_minus_utc();

        assert_ne!(winter_offset, summer_offset);
    }
}

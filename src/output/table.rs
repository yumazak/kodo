//! Table output formatter

use crate::error::Result;
use crate::output::Formatter;
use crate::stats::AnalysisResult;
use comfy_table::{Table, presets::UTF8_FULL};

/// Table output formatter
pub struct TableFormatter;

impl TableFormatter {
    /// Create a new table formatter
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

fn format_with_commas_u64(value: u64) -> String {
    let s = value.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

fn format_with_commas_i64(value: i64) -> String {
    if value < 0 {
        format!("-{}", format_with_commas_u64(value.unsigned_abs()))
    } else {
        format_with_commas_u64(value.unsigned_abs())
    }
}

impl Default for TableFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for TableFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_header(["Period", "Commits", "+Lines", "-Lines", "Net", "Files"]);

        for stat in &result.stats {
            table.add_row([
                stat.label.clone(),
                format_with_commas_u64(u64::from(stat.commits)),
                format_with_commas_u64(stat.additions),
                format_with_commas_u64(stat.deletions),
                format_with_commas_i64(stat.net_lines),
                format_with_commas_u64(u64::from(stat.files_changed)),
            ]);
        }

        let total = &result.total;
        table.add_row([
            "TOTAL".to_string(),
            format_with_commas_u64(u64::from(total.commits)),
            format_with_commas_u64(total.additions),
            format_with_commas_u64(total.deletions),
            format_with_commas_i64(total.net_lines),
            format_with_commas_u64(u64::from(total.files_changed)),
        ]);

        Ok(table.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::{AnalysisResult, PeriodStats};
    use chrono::NaiveDate;

    fn make_result() -> AnalysisResult {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let stats = vec![PeriodStats {
            label: "2024-01-01".to_string(),
            date: from,
            commits: 2,
            additions: 20,
            deletions: 5,
            net_lines: 15,
            files_changed: 3,
        }];

        AnalysisResult::new(
            "test-repo".to_string(),
            "daily".to_string(),
            from,
            to,
            stats,
        )
    }

    #[test]
    fn test_table_formatter_draws_utf8_border_and_total() {
        let formatter = TableFormatter::new();
        let result = make_result();
        let table = formatter.format(&result).unwrap();

        assert!(table.contains('┌'));
        assert!(table.contains('┬'));
        assert!(table.contains('┐'));
        assert!(table.contains("Period"));
        assert!(table.contains("Commits"));
        assert!(table.contains("TOTAL"));
    }

    #[test]
    fn test_table_formatter_formats_numbers_with_commas() {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let stats = vec![PeriodStats {
            label: "2024-01-01".to_string(),
            date: from,
            commits: 1_000,
            additions: 1_234_567,
            deletions: 12_345,
            net_lines: -1_234_567,
            files_changed: 9_999,
        }];

        let result = AnalysisResult::new(
            "test-repo".to_string(),
            "daily".to_string(),
            from,
            to,
            stats,
        );
        let formatter = TableFormatter::new();
        let table = formatter.format(&result).unwrap();

        assert!(table.contains("1,000"));
        assert!(table.contains("1,234,567"));
        assert!(table.contains("12,345"));
        assert!(table.contains("-1,234,567"));
        assert!(table.contains("9,999"));
    }
}

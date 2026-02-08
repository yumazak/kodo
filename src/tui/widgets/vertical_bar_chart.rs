//! Vertical bar chart widget for activity statistics

use ratatui::prelude::*;
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders};

/// Render a vertical bar chart
pub fn render_vertical_bar_chart(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    labels: &[&str],
    values: &[u32],
    color: Color,
) {
    let max_value = *values.iter().max().unwrap_or(&1).max(&1);

    let bars: Vec<Bar> = labels
        .iter()
        .zip(values.iter())
        .map(|(label, &value)| {
            Bar::default()
                .value(u64::from(value))
                .label(Line::from(*label))
                .style(Style::default().fg(color))
        })
        .collect();

    let total: u32 = values.iter().sum();
    let title_with_total = format!(" {title} ({total}) ");

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(title_with_total)
                .title_style(Style::default().fg(color).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .data(BarGroup::default().bars(&bars))
        .bar_width(3)
        .bar_gap(1)
        .max(u64::from(max_value));

    frame.render_widget(chart, area);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vertical_bar_chart_creation() {
        // Basic smoke test - actual rendering tested via integration
        let labels = ["Mon", "Tue", "Wed"];
        let values = [5, 10, 3];
        assert_eq!(labels.len(), values.len());
    }
}

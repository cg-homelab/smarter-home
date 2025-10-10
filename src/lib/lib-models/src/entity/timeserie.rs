use super::power::PowerMetrics;

pub enum TimeSerieRecord {
    PowerMetrics(PowerMetrics),
}

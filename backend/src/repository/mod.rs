use chrono::{DateTime, Utc};
use questdb::{
    ingress::{Sender as QuestSender, SenderBuilder, TimestampNanos},
    Result,
};
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;
mod mongo;
mod quest;

pub struct InfluxLogEvent {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub running: bool,
}

pub enum InfluxMessage {
    NewConsumptionMetric,
}

pub struct NewConsumptionMetric {
    pub home_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub power: f64,
    pub min_power: f64,
    pub max_power: f64,
    pub average_power: f64,
    pub last_meter_consumption: f64,
    pub last_meter_production: f64,
    pub accumulated_consumption_today: f64,
    pub accumulated_production_today: f64,
    pub accumulated_consumption_hour: f64,
    pub accumulated_production_hour: f64,
    pub current_price: f64,
    pub accumulated_cost_today: f64,
}
pub struct InfluxActor {
    influx_sender_client: QuestSender,
    host: String,
    port: u16,
    pub tx_data: Sender<InfluxMessage>,
    rx_data: Receiver<InfluxMessage>,
    pub rx_log: Receiver<InfluxLogEvent>,
    tx_log: Sender<InfluxLogEvent>,
}
impl InfluxActor {
    pub fn new<T>(host: &str, port: u16) -> Self {
        let sender = SenderBuilder::new(host, port)
            .connect()
            .expect("Can't connect to database");

        let (tx_data, rx_data): (Sender<InfluxMessage>, Receiver<InfluxMessage>) =
            tokio::sync::mpsc::channel(1000);

        let (tx_log, rx_log): (Sender<InfluxLogEvent>, Receiver<InfluxLogEvent>) =
            tokio::sync::mpsc::channel(1000);

        Self {
            influx_sender_client: sender,
            host: host.to_string(),
            port,
            tx_data,
            rx_data,
            rx_log,
            tx_log,
        }
    }
}

pub struct Repository {
    influx_sender: QuestSender,
}

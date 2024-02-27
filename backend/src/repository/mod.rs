use questdb::{
    ingress::{Sender as QuestSender, SenderBuilder, TimestampNanos},
    Result,
};
mod mongo;
mod quest;

pub struct Repository {
    influx_sender: QuestSender,
}

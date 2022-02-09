use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions},
    types::FieldTable,
    ExchangeKind,
};
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Value;
use tokio_stream::{Stream, StreamExt};
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::event::DispatchEventWithTypeDeserializer;
use twilight_model::gateway::OpCode;

use crate::Config;
use sushii_interactions::error::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayloadInfo {
    pub op: OpCode,
    pub t: Option<String>,
    pub d: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old: Option<Value>,
}

pub async fn get_events(config: &Config) -> Result<impl Stream<Item = Result<DispatchEvent>>> {
    let amqp = lapin::Connection::connect(
        format!(
            "amqp://{}:{}@{}:{}/%2f",
            config.rabbit.username, config.rabbit.password, config.rabbit.host, config.rabbit.port
        )
        .as_str(),
        lapin::ConnectionProperties::default(),
    )
    .await?;

    let channel = amqp.create_channel().await?;

    channel
        .exchange_declare(
            "gateway",
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                passive: false,
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "gateway.recv",
            "sushii_interactions",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(async_stream::try_stream! {
        while let Some(message) = consumer.next().await {
            match message {
                Ok((channel, delivery)) => {
                    let _ = channel
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .await;

                    let payload: PayloadInfo = serde_json::from_slice(&delivery.data.as_slice())?;

                    let event_type = match payload.t {
                        Some(t) => t,
                        None => {
                            tracing::warn!("Payload missing t: {:?}", payload);
                            continue;
                        }
                    };

                    let de = DispatchEventWithTypeDeserializer::new(&event_type);
                    let gateway_event = de
                        .deserialize(payload.d)?;

                    yield gateway_event;
                },
                Err(e) => {
                    tracing::error!("Failed to consume delivery: {}?", e);
                }
            }
        }
    })
}

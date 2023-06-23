use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

use crate::discord::{send_alarm_webhook, send_ok_webhook};

mod discord;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    tracing::info!("starting up...");

    lambda_runtime::run(service_fn(|event: LambdaEvent<Value>| async move {
        tracing::info!("event: {:#?}", event);

        // Get first record SNS message body
        let payload: Value = event
            .payload
            .get("Records")
            .and_then(|records| records.get(0))
            .and_then(|record| record.get("Sns"))
            .and_then(|sns| sns.get("Message"))
            .and_then(|message| message.as_str())
            .and_then(|message| serde_json::from_str(message).ok())
            .unwrap_or_default();

        tracing::info!("payload: {:#?}", payload);

        match payload.get("NewStateValue") {
            Some(Value::String(state)) if state == "ALARM" => {
                send_alarm_webhook(&payload).await?;
            }
            Some(Value::String(state)) if state == "OK" => {
                send_ok_webhook(&payload).await?;
            }
            _ => {
                tracing::info!("Unknown state: {:#?}", payload);
            }
        }

        Ok::<_, Error>(())
    }))
    .await?;

    Ok(())
}

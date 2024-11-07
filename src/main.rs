use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use tracing::info;

mod discord;
use discord::{send_webhook, WebhookPayload};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    info!("Lambda function starting up");

    lambda_runtime::run(service_fn(handle_event)).await
}

async fn handle_event(event: LambdaEvent<SnsEvent>) -> Result<(), Error> {
    for record in event.payload.records {
        if let Err(e) = process_record(&record.sns.message).await {
            tracing::error!("Failed to process record: {}", e);
        }
    }
    Ok(())
}

async fn process_record(message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let alarm: Value = serde_json::from_str(message)?;
    let webhook_payload = WebhookPayload::from_alarm(&alarm)?;
    send_webhook(&webhook_payload).await?;
    Ok(())
}

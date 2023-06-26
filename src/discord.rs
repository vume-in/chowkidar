use anyhow::Error;
use serde_json::{json, Value};

pub async fn send_alarm_webhook(payload: &Value) -> Result<(), Error> {
    let webhook_url = std::env::var("WEBHOOK_URL")
        .expect("A WEBHOOK_URL must be set in this app's Lambda environment variables.");

    // Construct Discord webhook payload
    let discord_payload = json!({
        "content": format!("ALARM: {}", payload.get("AlarmName").unwrap_or(&Value::Null)),
        "embeds": [
          {
            "title": payload.get("AlarmDescription").unwrap_or(&Value::Null),
            "color": 16_725_558,
            "fields": [
              {
                "name": "Trigger Time",
                "value": payload.get("StateChangeTime").unwrap_or(&Value::Null),
              },
            ]
          }
        ]
    });

    let client = reqwest::Client::new();
    let res = client
        .post(&webhook_url)
        .json(&discord_payload)
        .send()
        .await?;

    tracing::info!("Discord response: {:#?}", res);

    Ok(())
}

pub async fn send_ok_webhook(payload: &Value) -> Result<(), Error> {
    let webhook_url = std::env::var("WEBHOOK_URL")
        .expect("A WEBHOOK_URL must be set in this app's Lambda environment variables.");

    // Construct Discord webhook payload
    let discord_payload = json!({
        "content": format!("OK: {}", payload.get("AlarmName").unwrap_or(&Value::Null)),
        "embeds": [
          {
            "title": payload.get("AlarmDescription").unwrap_or(&Value::Null),
            "color": 2_031_360,
            "fields": [
              {
                "name": "Trigger Time",
                "value": payload.get("StateChangeTime").unwrap_or(&Value::Null),
              },
            ]
          }
        ]
    });

    let client = reqwest::Client::new();
    let res = client
        .post(&webhook_url)
        .json(&discord_payload)
        .send()
        .await?;

    tracing::info!("Discord response: {:#?}", res);

    Ok(())
}

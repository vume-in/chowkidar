use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

#[derive(Debug, Serialize)]
pub struct WebhookPayload {
    content: String,
    embeds: Vec<Embed>,
}

#[derive(Debug, Serialize)]
struct Embed {
    title: Option<String>,
    color: u32,
    fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
struct Field {
    name: String,
    value: String,
}

impl WebhookPayload {
    pub fn from_alarm(alarm: &Value) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let alarm_name = alarm["AlarmName"].as_str().unwrap_or("Unknown Alarm");
        let alarm_description = alarm["AlarmDescription"]
            .as_str()
            .unwrap_or("No description");
        let new_state_value = alarm["NewStateValue"].as_str().unwrap_or("Unknown");
        let state_change_time = alarm["StateChangeTime"].as_str().unwrap_or("Unknown time");
        let new_state_reason = alarm["NewStateReason"]
            .as_str()
            .unwrap_or("No reason provided");
        let region = alarm["Region"].as_str().unwrap_or("Unknown region");

        let color = match new_state_value {
            "ALARM" => 0xFF0000,             // Red
            "OK" => 0x00FF00,                // Green
            "INSUFFICIENT_DATA" => 0xFFFF00, // Yellow
            _ => 0x0000FF,                   // Blue for unknown states
        };

        let aws_console_link = format!(
            "https://console.aws.amazon.com/cloudwatch/home?region={}#alarmsV2:alarm/{}",
            region, alarm_name
        );

        Ok(Self {
            content: format!(
                "{}: {}",
                match new_state_value {
                    "ALARM" => "🚨 ALARM",
                    "OK" => "✅ OK",
                    "INSUFFICIENT_DATA" => "⚠️ INSUFFICIENT DATA",
                    _ => "ℹ️ UNKNOWN STATE",
                },
                alarm_name
            ),
            embeds: vec![Embed {
                title: Some(alarm_description.to_string()),
                color,
                fields: vec![
                    Field {
                        name: "State".to_string(),
                        value: new_state_value.to_string(),
                    },
                    Field {
                        name: "Reason".to_string(),
                        value: new_state_reason.to_string(),
                    },
                    Field {
                        name: "Trigger Time".to_string(),
                        value: state_change_time.to_string(),
                    },
                    Field {
                        name: "AWS Console Link".to_string(),
                        value: aws_console_link,
                    },
                ],
            }],
        })
    }
}

pub async fn send_webhook(
    payload: &WebhookPayload,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let webhook_url = std::env::var("WEBHOOK_URL")?;

    let response = CLIENT.post(&webhook_url).json(payload).send().await?;

    if !response.status().is_success() {
        tracing::error!(
            "Discord webhook failed with status {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    Ok(())
}

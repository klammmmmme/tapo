/// H100 Example
use std::env;

use log::{info, LevelFilter};
use tapo::{responses::ChildDeviceResult, ApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    let hub = ApiClient::new(tapo_username, tapo_password)?
        .h100(ip_address)
        .await?;

    let device_info = hub.get_device_info().await?;
    info!("Device info: {device_info:?}");

    info!("Getting child devices...");
    let child_device_list = hub.get_child_device_list().await?;

    for child in child_device_list {
        match child {
            ChildDeviceResult::S200B(device) => {
                let trigger_logs = device.get_trigger_logs(&hub, 5, 0).await?;

                info!(
                    "Found S200B child device with nickname: {}, id: {}, last 5 trigger logs: {:?}",
                    device.nickname, device.device_id, trigger_logs
                );
            }
            ChildDeviceResult::T100(device) => {
                let trigger_logs = device.get_trigger_logs(&hub, 5, 0).await?;

                info!(
                    "Found T100 child device with nickname: {}, id: {}, detected: {}, last 5 trigger logs: {:?}",
                    device.nickname, device.device_id, device.detected, trigger_logs
                );
            }
            ChildDeviceResult::T110(device) => {
                let trigger_logs = device.get_trigger_logs(&hub, 5, 0).await?;

                info!(
                    "Found T110 child device with nickname: {}, id: {}, open: {}, last 5 trigger logs: {:?}",
                    device.nickname, device.device_id, device.open, trigger_logs
                );
            }
            ChildDeviceResult::T310(device) | ChildDeviceResult::T315(device) => {
                info!(
                    "Found T31X child device with nickname: {}, id: {}, temperature: {} {:?}, humidity: {}%",
                    device.nickname,
                    device.device_id,
                    device.current_temperature,
                    device.temperature_unit,
                    device.current_humidity
                );
            }
            _ => {
                info!("Found unsupported device.")
            }
        }
    }

    Ok(())
}

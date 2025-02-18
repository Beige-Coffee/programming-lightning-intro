use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Represents a Lightning channel's state
#[derive(Debug, Clone)]
struct Channel {
    id: String,
    funding_amt: u64,
    state: String, // Example states: "open_channel", "accept_channel", etc.
}

/// Global storage for channels and a counter for unique 3-digit IDs
type ChannelMap = Arc<Mutex<HashMap<String, Channel>>>;
type ChannelCounter = Arc<Mutex<u16>>; // Keeps track of the next available ID

/// Runs the peer simulator with multiple channel tracking
pub async fn run(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to port");

    let channels: ChannelMap = Arc::new(Mutex::new(HashMap::new()));
    let channel_counter: ChannelCounter = Arc::new(Mutex::new(100)); // Start at 100

    println!("🚀 Listening for peer messages on port {}", port);

    loop {
        let (mut socket, peer_addr) = listener.accept().await.expect("Failed to accept connection");

        let peer_ip = peer_addr.ip().to_string();
        if !peer_ip.starts_with("127.") {
            continue; // Silently ignore non-local connections
        }

        println!("✅ New peer connected from {}", peer_ip);

        let channels = channels.clone();
        let channel_counter = channel_counter.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                match socket.readable().await {
                    Ok(_) => match socket.try_read(&mut buf) {
                        Ok(0) => {
                            println!("🔌 Peer {} disconnected", peer_ip);
                            break;
                        }
                        Ok(n) => {
                            let received_data = String::from_utf8_lossy(&buf[..n]).trim().to_string();

                            if received_data.is_empty() {
                                continue;
                            }

                            println!("📩 Received valid command from {}: {}", peer_ip, received_data);

                            let response = handle_message(&received_data, &channels, &channel_counter);

                            if let Err(e) = socket.write_all(response.as_bytes()).await {
                                println!("⚠️ Error sending response: {}", e);
                                break;
                            }
                        }
                        Err(_) => continue,
                    },
                    Err(_) => break,
                }
            }
        });
    }
}

/// Handles messages and enforces correct state transitions
fn handle_message(msg: &str, channels: &ChannelMap, channel_counter: &ChannelCounter) -> String {
    let parts: Vec<&str> = msg.split_whitespace().collect();
    if parts.is_empty() {
        return "ERROR: Empty message\n".to_string();
    }

    let command = parts[0];

    match command {
        "HELP" => {
            return "\
Available Commands:
- OPEN_CHANNEL <funding_amt>: Opens a new channel with the specified funding amount.
- FUNDING_CREATED <channel_id>: Moves the channel to the 'funding_created' state.
- FUNDING_SIGNED <channel_id>: Moves the channel to the 'funding_signed' state.
- FUNDING_LOCKED <channel_id>: Moves the channel to the 'funding_locked' state, activating it.
- GET_CHANNELS: Lists all active channels and their states.
- HELP: Displays this help message.
".to_string();
        }

        "OPEN_CHANNEL" => {
            if parts.len() != 2 {
                return "ERROR: Usage: OPEN_CHANNEL <funding_amt>\n".to_string();
            }

            let funding_amt: u64 = parts[1].parse().unwrap_or(0);

            let mut channels_lock = channels.lock().unwrap();
            let mut counter = channel_counter.lock().unwrap();

            let channel_id = format!("{:03}", *counter);
            *counter += 1;

            channels_lock.insert(
                channel_id.clone(),
                Channel {
                    id: channel_id.clone(),
                    funding_amt,
                    state: "open_channel".to_string(),
                },
            );

            format!(
                "ACCEPT_CHANNEL: id={} funding={}\n",
                channel_id, funding_amt
            )
        }

        "FUNDING_CREATED" if parts.len() == 2 => {
            let channel_id = parts[1].to_string();
            let mut channels_lock = channels.lock().unwrap();
            if let Some(channel) = channels_lock.get_mut(&channel_id) {
                if channel.state != "open_channel" {
                    return format!(
                        "ERROR: Cannot move to funding_created, current state is {}\n",
                        channel.state
                    );
                }
                channel.state = "funding_created".to_string();
                return format!("FUNDING_SIGNED: id={}\n", channel_id);
            }
            "ERROR: Channel not found\n".to_string()
        }

        "FUNDING_SIGNED" if parts.len() == 2 => {
            let channel_id = parts[1].to_string();
            let mut channels_lock = channels.lock().unwrap();
            if let Some(channel) = channels_lock.get_mut(&channel_id) {
                if channel.state != "funding_created" {
                    return format!(
                        "ERROR: Cannot move to funding_signed, current state is {}\n",
                        channel.state
                    );
                }
                channel.state = "funding_signed".to_string();
                return format!("FUNDING_LOCKED: id={}\n", channel_id);
            }
            "ERROR: Channel not found\n".to_string()
        }

        "FUNDING_LOCKED" if parts.len() == 2 => {
            let channel_id = parts[1].to_string();
            let mut channels_lock = channels.lock().unwrap();
            if let Some(channel) = channels_lock.get_mut(&channel_id) {
                if channel.state != "funding_signed" {
                    return format!(
                        "ERROR: Cannot move to funding_locked, current state is {}\n",
                        channel.state
                    );
                }
                channel.state = "funding_locked".to_string();
                return format!("CHANNEL {} IS NOW ACTIVE!\n", channel_id);
            }
            "ERROR: Channel not found\n".to_string()
        }

        "GET_CHANNELS" => {
            let channels_lock = channels.lock().unwrap();
            if channels_lock.is_empty() {
                return "No active channels\n".to_string();
            }

            let mut response = String::from("Active Channels:\n");
            for (id, channel) in channels_lock.iter() {
                response.push_str(&format!(
                    "id={} funding={} state={}\n",
                    id, channel.funding_amt, channel.state
                ));
            }
            response
        }

        _ => "UNKNOWN COMMAND. Type HELP for available commands.\n".to_string(),
    }
}

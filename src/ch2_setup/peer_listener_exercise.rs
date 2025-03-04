#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;

/// Represents a Lightning channel's state
#[derive(Debug, Clone)]
struct Channel {
    id: String,
    channel_id: Option<String>,
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
- OPEN_CHANNEL <funding_amt> <to_self_delay> <sats_p_vbyte>: Opens a new channel with the specified funding amount, other node's to_self_delay, and proposed fee-rates.
- FUNDING_CREATED <channel_id>: Moves the channel to the 'funding_created' state.
- FUNDING_SIGNED <channel_id>: Moves the channel to the 'funding_signed' state.
- FUNDING_LOCKED <channel_id>: Moves the channel to the 'funding_locked' state, activating it.
- GET_CHANNELS: Lists all active channels and their states.
- HELP: Displays this help message.
".to_string();
        }

        "OPEN_CHANNEL" => {
            if parts.len() != 4 {
                return "ERROR: Usage: OPEN_CHANNEL <funding_amt> <to_self_delay> <sats_p_vbyte>\n\n".to_string();
            }

            let funding_amt: u64 = parts[1].parse().unwrap_or(0);
            let to_self_delay: u64 = parts[2].parse().unwrap_or(0);
            let sats_p_vbyte: u64 = parts[3].parse().unwrap_or(0);

            if funding_amt < 100000 {
                return "REJECT CHANNEL: funding_amt too small. Will not accept channels with funding_amt less than 100,000 sats!\n\n".to_string();
            }

            if to_self_delay > 144 {
                return "REJECT CHANNEL: to_self_delay too large. Will not accept channels with to_self_delay larger than 144!\n\n".to_string();
            }

            if sats_p_vbyte < 10 {
                return "REJECT CHANNEL: sats_p_vbyte too small. Will not accept channels with sats_p_vbyte less than 10 sats/vByte!\n\n".to_string();
            }

            let mut channels_lock = channels.lock().unwrap();
            let mut counter = channel_counter.lock().unwrap();

            let channel_id = format!("{:03}", *counter);
            *counter += 1;

            channels_lock.insert(
                channel_id.clone(),
                Channel {
                    id: channel_id.clone(),
                    channel_id: None,
                    funding_amt,
                    state: "open_channel".to_string(),
                },
            );

            format!(
                "ACCEPT_CHANNEL: temp_channel_id={} to_self_delay=144, channel_keys[...]\n",
                channel_id,
            )
        }

        "FUNDING_CREATED" if parts.len() == 4 => {
            
            let channel_id = parts[1].to_string();
            let funding_tx = parts[2].to_string();
            let signature = parts[3].to_string();

            if is_valid_txid(&funding_tx) != true {
                return format!(
                    "ERROR: MUST provide funding output in following format:  TxID:OutputIndex\n\n"
                );
            }
            
            let mut channels_lock = channels.lock().unwrap();
            if let Some(channel) = channels_lock.get_mut(&channel_id) {
                if channel.state != "open_channel" {
                    return format!(
                        "ERROR: Cannot move to funding_created, current state is {}\n",
                        channel.state
                    );
                }
                channel.state = "funding_signed".to_string();
                channel.channel_id = Some(generate_random_channel_id());
                return format!("FUNDING_SIGNED: channel_id={}, signature=[avb1adx4]\n\n", channel.channel_id.as_ref().unwrap());
            }
            "ERROR: Channel not found\n".to_string()
        }
        "CHANNEL_READY" if parts.len() == 2 => {
                let channel_id = parts[1].to_string(); 
                let mut channels_lock = channels.lock().unwrap();

                // Find the channel with the matching channel_id
                if let Some((_, channel)) = channels_lock.iter_mut().find(|(_, ch)| {
                    ch.channel_id.as_ref() == Some(&channel_id)
                }) {
                    if channel.state != "funding_signed" {
                        return format!(
                            "ERROR: Cannot is not ready, current state is {}\n",
                            channel.state
                        );
                    }
                    channel.state = "channel_ready".to_string();
                    return format!("CHANNEL_READY: id={}\n", channel.channel_id.as_ref().unwrap());
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
                    "channel_id={:?}, temp_channel_id={} funding={} state={}\n",
                    channel.channel_id, id, channel.funding_amt, channel.state
                ));
            }
            response
        }

        _ => "UNKNOWN COMMAND. Type HELP for available commands.\n".to_string(),
    }
}

fn is_valid_txid(s: &str) -> bool {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 { // Must have exactly two parts (before and after ":")
        return false;
    }
    let before_colon = parts[0];
    let after_colon = parts[1];

    // Before colon: any text (non-empty in this case, adjust as needed)
    if before_colon.is_empty() {
        return false;
    }

    // After colon: must be a valid number (integer)
    after_colon.parse::<i64>().is_ok()
}

// Generate a random 3-letter channel ID
fn generate_random_channel_id() -> String {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    let mut id = String::with_capacity(3);
    for _ in 0..3 {
        let idx = rng.gen_range(0, letters.len()); // Two arguments: low, high
        id.push(letters.chars().nth(idx).unwrap());
    }
    id
}
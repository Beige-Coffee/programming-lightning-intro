#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use tokio::net::TcpListener;

// Global counter for tracking calls to setup_inbound
pub static mut CALL_COUNT: usize = 0;

// Fake PeerManager struct to simulate a Lightning peer manager
#[derive(Clone)]
pub struct PeerManager {
    pub id: String, // A simple identifier for the peer manager
}

// Pretend this is a Lightning function that handles inbound connections
pub async fn setup_inbound(peer_manager: PeerManager, tcp_stream: tokio::net::TcpStream) {
    assert_eq!(peer_manager.id, "test_node", "PeerManager ID should be 'test_node'");
    assert!(tcp_stream.peer_addr().is_ok(), "TcpStream should be valid");
    unsafe { CALL_COUNT += 1; } // Increment the global counter
}

pub async fn start_listener(peer_manager: PeerManager) {
    let port = 9735;
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind");

    loop {
        let (tcp_stream, _) = listener.accept()
            .await
            .expect("Failed to accept");

        setup_inbound(peer_manager.clone(), tcp_stream).await;
    }
}
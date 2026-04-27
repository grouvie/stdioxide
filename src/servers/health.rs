//! Health check TCP server that accepts and immediately drops connections.

use std::net::TcpListener;

use tracing::warn;

/// Waits for clients to connect on the `health` port, and immediately drops any connections. The existence
/// of a successful connection is used by the client as a health check for whether the process is alive.
pub fn health_server(listener: TcpListener) -> Result<(), anyhow::Error> {
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Immediately drop it; successful connect is enough.
            }
            Err(e) => {
                warn!("[health] accept failed: {e}");
            }
        }
    }

    Ok(())
}

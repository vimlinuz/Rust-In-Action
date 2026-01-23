#![allow(unused)]
use std::error::Error;

use zbus::{Connection, proxy, zvariant::Value};

#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    fn Enable(&self, enable: bool) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let proxy = NetworkManagerProxy::new(&connection).await?;

    proxy.Enable(true).await?;

    Ok(())
}

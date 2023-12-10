use once_cell::sync::Lazy;
use std::env;
use tiberius::{Client, Config};
use async_std::net::TcpStream;
use tiberius::error::Error;

static CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:127.0.0.1\\SQL2022D,23241;database=DestinationDB;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
    })
});


async fn connect() -> Result<Client<TcpStream>, Error> {

    let config = Config::from_ado_string(&CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    return Ok(client)
}
use darkruby_tictactoe::server::connection::Connection;
use darkruby_tictactoe::server::game_server::GameServer;
use std::error::Error;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let address = "0.0.0.0:6789";
  let listener = TcpListener::bind(address).await?;

  let (transmitter, mut receiver) = mpsc::channel(32);

  tokio::spawn(async move {
    let mut game_server = GameServer::new();
    game_server.respond(&mut receiver).await;
  });

  loop {
    let (stream, address) = listener.accept().await?;
    let transmitter = transmitter.clone();
    tokio::spawn(async move {
      let mut connection = Connection::new(stream, address);
      match GameServer::process(&mut connection, transmitter).await {
        Err(e) => println!("{}", e),
        _ => (),
      }
    });
  }
}

use darkruby_tictactoe::server::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

#[tokio::main]
async fn main() {
  let address = "127.0.0.1:6789";
  let mut listener = TcpListener::bind(address).await.unwrap();

  loop {
    let (mut socket, _) = listener.accept().await.unwrap();
    tokio::spawn(async move {
      // match process_request(&mut socket).await {
      //     Err(e) => println!("Err {}", e),
      //     _ => (),
      // }
    });
  }
}

use darkruby_tictactoe::network::connection::Connection;
use darkruby_tictactoe::network::server::Server;
use std::error::Error;
use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

#[derive(StructOpt, Debug)]
#[structopt(name = "tactactoe-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "tictactoe")]
struct Cli {
  #[structopt(name = "hostname", long = "--host", default_value = "0.0.0.0")]
  host: String,

  #[structopt(name = "port", long = "--port", default_value = "6789")]
  port: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::from_args();
  let addr = format!("{}:{}", cli.host, cli.port);
  let listener = TcpListener::bind(addr).await?;

  let (transmitter, mut receiver) = mpsc::channel(32);

  tokio::spawn(async move {
    let mut server = Server::new();
    server.respond(&mut receiver).await;
  });

  loop {
    let (stream, address) = listener.accept().await?;
    let transmitter = transmitter.clone();
    tokio::spawn(async move {
      let mut connection = Connection::new(stream, address);
      match Server::process(&mut connection, transmitter).await {
        Err(e) => println!("{}", e),
        _ => (),
      }
    });
  }
}

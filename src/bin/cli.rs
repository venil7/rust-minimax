use darkruby_tictactoe::network::client::Client;
use darkruby_tictactoe::state::State;
use std::error::Error;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tactactoe-client", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "tictactoe")]
struct Cli {
  #[structopt(name = "hostname", long = "--host", default_value = "127.0.0.1")]
  host: String,

  #[structopt(name = "port", long = "--port", default_value = "6789")]
  port: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::from_args();
  let addr = format!("{}:{}", cli.host, cli.port);
  let mut client = Client::connect(&addr).await?;
  client.new_game().await?;

  println!("{}", client.board());

  loop {
    println!("Your move: [0-8]: ");
    let mut position = String::new();
    io::stdin().read_line(&mut position)?;

    let position: u8 = match position.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match client.request_move(position).await {
      Ok(_) => println!("{}", client.board()),
      Err(game_error) => {
        println!("{}\n{}\n", game_error, client.board());
        continue;
      }
    };

    if let State::GameOver(winner) = client.board().state() {
      println!("{} wins the game", winner);
      client.new_game().await?;
    }
  }

  Ok(())
}

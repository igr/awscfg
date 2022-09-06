use clap::{Parser, Subcommand};

pub mod arn;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {  
  #[clap(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// ARN related commands
  Arn {
    /// lists test values
    #[clap(short, long, action)]
    profile: String,
  },
}


fn main() {
  let cli = Cli::parse();
  
  match &cli.command {
    Some(Commands::Arn { profile }) => {
      let arn = arn::parse_arn_of_profile(profile);
      print!("{arn}");
    }
    None => {}
  }
}
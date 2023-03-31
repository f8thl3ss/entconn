use clap::Parser;
use command::{connect, find_cli, CommandArgs};

mod command;

fn main() {
    let args = CommandArgs::parse();
    if let Err(error) = find_cli() {
        println!("{}", error);
        return;
    }
    let out = connect(args).output().expect("Failed to execute nmcli");
    if out.stderr.is_empty() {
        println!("Could not connect.");
        println!("{}", String::from_utf8_lossy(&out.stdout));
    } else {
        println!("Connected!");
    }
}

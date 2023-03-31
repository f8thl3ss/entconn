use clap::Parser;
use std::{io::ErrorKind, process::Command};

pub fn find_cli() -> Result<bool, String> {
    match Command::new("nmcli").arg("-v").output() {
        Ok(_) => Ok(true),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err("Could not find nmcli".to_string()),
            _ => Err(e.to_string()),
        },
    }
}

pub fn connect(args: CommandArgs) -> Command {
    let mut command = Command::new("nmcli");
    command
        .arg("connection")
        .arg("add")
        .arg("type")
        .arg("wifi")
        .arg("connection.id");
    if let Some(id) = args.id {
        command.arg(id);
    } else {
        command.arg(args.ssid.clone());
    };
    command.arg("wifi.ssid").arg(args.ssid);
    command.arg("wifi.mode").arg("infrastructure");
    command.arg("wifi-sec.key-mgmt").arg("wpa-eap");
    command.arg("802-1x.eap").arg("peap");
    command.arg("802-1x.identity").arg(args.identity);
    command.arg("802-1x.phase2-auth").arg("mschapv2");
    command.arg("802-1x.password").arg(args.password);
    command
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandArgs {
    /// Wifi id
    #[arg(long)]
    id: Option<String>,

    /// Wifi SSID
    #[arg(short, long)]
    ssid: String,

    /// Enterprise network identity (802-1x identity)
    #[arg(short, long)]
    identity: String,

    /// Enterprise network password (802-1x password)
    #[arg(short, long)]
    password: String,
}

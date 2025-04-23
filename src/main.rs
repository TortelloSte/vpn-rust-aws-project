use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;

mod aws;
mod config;
mod ssh;
mod wireguard;

use aws::InstanceInfo;

fn main() {
    dotenv::dotenv().ok();
    config::check_env();

    println!("➡️  Avvio creazione istanza EC2 nella zona designata...");
    let instance = aws::get_or_create_instance();

    println!("✅ Istanza creata: {}", instance.public_ip);

    println!("🔐 Connessione via SSH per configurare WireGuard...");
    ssh::provision_server(&instance);

    println!("📦 Generazione config client...");
    wireguard::generate_client_config(&instance);

    println!("🎉 VPN pronta! Ora puoi connetterti con WireGuard.");
}

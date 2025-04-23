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
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    config::check_env();

    // Starting EC2 instance creation in designated region...
    println!("Starting EC2 instance creation in the designated region...");
    let instance = aws::get_or_create_instance();

    // Instance successfully created
    println!("Instance created: {}", instance.public_ip);

    // Connect via SSH to configure WireGuard
    println!("Connecting via SSH to configure WireGuard...");
    ssh::provision_server(&instance);

    // Generate WireGuard client configuration
    println!("Generating client config...");
    wireguard::generate_client_config(&instance);

    // VPN setup complete
    println!("VPN ready! You can now connect using WireGuard.");
}
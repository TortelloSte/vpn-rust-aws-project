use crate::InstanceInfo;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::fs;

// Function to provision the EC2 instance with WireGuard
pub fn provision_server(instance: &InstanceInfo) {
    let ip = &instance.public_ip;
    let ssh_key_path = std::env::var("SSH_KEY_PATH").expect("SSH_KEY_PATH not set in .env");

    // Wait for SSH accessibility (60s)
    println!("Waiting for SSH accessibility (60s)...");
    sleep(Duration::from_secs(60));

    let remote_script = r#"
        sudo yum update -y

        # Install WireGuard
        sudo yum install -y wireguard-tools

        # Create directory if it doesn't exist
        sudo mkdir -p /etc/wireguard

        # Generate key pairs
        wg genkey | sudo tee /etc/wireguard/server_private.key | wg pubkey | sudo tee /etc/wireguard/server_public.key

        # Enable IP forwarding
        echo 'net.ipv4.ip_forward = 1' | sudo tee -a /etc/sysctl.conf
        sudo sysctl -p
    "#;

    let remote_command = format!(
        "ssh -o StrictHostKeyChecking=no -i {} ec2-user@{} '{}'",
        ssh_key_path, ip, remote_script
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(&remote_command)
        .status()
        .expect("Error during remote execution via SSH");

    if !status.success() {
        panic!("SSH provisioning failed");
    }

    println!("🔧 WireGuard installed, keys generated, and service active on {}", ip);

    let fetch_key_command = format!(
        "ssh -o StrictHostKeyChecking=no -i {} ec2-user@{} 'sudo cat /etc/wireguard/server_public.key'",
        ssh_key_path, ip
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(fetch_key_command)
        .output()
        .expect("Error retrieving server public key");

    let pubkey = String::from_utf8_lossy(&output.stdout).trim().to_string();
    fs::create_dir_all("out").ok();
    fs::write("out/server_public.key", &pubkey).expect("Unable to save server public key");

    println!("Server public key saved to out/server_public.key");
}
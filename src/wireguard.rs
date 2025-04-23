use crate::InstanceInfo;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

// Function to generate the WireGuard client configuration
pub fn generate_client_config(instance: &InstanceInfo) {
    let ip = &instance.public_ip;

    let key_path = Path::new("out/server_public.key");

    if !key_path.exists() {
        // File missing, likely SSH provisioning failed
        panic!("The server_public.key file does not exist. Check that SSH provisioning completed successfully.");
    }

    let server_pubkey = fs::read_to_string(key_path)
        .expect("Error reading server public key");

    let private_key = String::from_utf8(
        Command::new("wg").arg("genkey").output().unwrap().stdout
    ).unwrap();

    let public_key = String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo {} | wg pubkey", private_key.trim()))
            .output()
            .unwrap()
            .stdout
    ).unwrap();

    let client_conf = format!(
        "[Interface]\nPrivateKey = {}\nAddress = 10.0.0.2/24\nDNS = 1.1.1.1\n\n[Peer]\nPublicKey = {}\nEndpoint = {}:51820\nAllowedIPs = 0.0.0.0/0\nPersistentKeepalive = 25\n",
        private_key.trim(),
        server_pubkey.trim(),
        ip
    );

    let out_dir = Path::new("out");
    fs::create_dir_all(out_dir).unwrap();

    let mut file = File::create(out_dir.join("client.conf")).unwrap();
    file.write_all(client_conf.as_bytes()).unwrap();

    // Configuration file successfully saved
    println!("client.conf file saved to ./out/");
}
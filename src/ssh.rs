use crate::InstanceInfo;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::fs;

pub fn provision_server(instance: &InstanceInfo) {
    let ip = &instance.public_ip;
    let ssh_key_path = std::env::var("SSH_KEY_PATH").expect("SSH_KEY_PATH non impostata nel .env");

    println!("⏳ Attendo accessibilità SSH (60s)...");
    sleep(Duration::from_secs(60));

    let remote_script = r#"
        sudo yum update -y

        # Installa WireGuard
        sudo yum install -y wireguard-tools

        # Crea la cartella se non esiste
        sudo mkdir -p /etc/wireguard

        # Genera chiavi
        wg genkey | sudo tee /etc/wireguard/server_private.key | wg pubkey | sudo tee /etc/wireguard/server_public.key

        # Abilita IP forwarding
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
        .expect("Errore nell'esecuzione remota via SSH");

    if !status.success() {
        panic!("❌ Fallito il provisioning SSH");
    }

    println!("🔧 WireGuard installato, chiavi generate e servizio attivo su {}", ip);

    let fetch_key_command = format!(
        "ssh -o StrictHostKeyChecking=no -i {} ec2-user@{} 'sudo cat /etc/wireguard/server_public.key'",
        ssh_key_path, ip
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(fetch_key_command)
        .output()
        .expect("Errore nel recupero della chiave pubblica server");

    let pubkey = String::from_utf8_lossy(&output.stdout).trim().to_string();
    fs::create_dir_all("out").ok();
    fs::write("out/server_public.key", &pubkey).expect("Impossibile salvare la chiave pubblica server");

    println!("🔑 Chiave pubblica server salvata in out/server_public.key");
}
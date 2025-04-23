use std::process::Command;

pub struct InstanceInfo {
    pub instance_id: String,
    pub public_ip: String,
}

pub fn get_or_create_instance() -> InstanceInfo {
    let output = Command::new("aws")
        .args([
            "ec2", "describe-instances",
            "--filters", "Name=tag:Name,Values=", "Name=instance-state-name,Values=running",
            "--query", "Reservations[0].Instances[0].[InstanceId,PublicIpAddress]",
            "--output", "text",
            "--region", "eu-south-1"
        ])
        .output()
        .expect("Failed to describe instances");

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if !result.is_empty() && !result.contains("None") {
        let parts: Vec<&str> = result.split_whitespace().collect();

        if parts.len() == 2 {
            println!("♻️ Istanza già esistente trovata.");
            return InstanceInfo {
                instance_id: parts[0].to_string(),
                public_ip: parts[1].to_string(),
            };
        }
    }

    // Altrimenti la crea
    create_instance()
}

pub fn create_instance() -> InstanceInfo {
    let output = Command::new("aws")
        .args([
            "ec2", "run-instances",
            "--image-id", "", // add ami
            "--instance-type", "t3.micro",
            "--key-name", "", // add key name
            "--security-groups", "default",
            "--region", "eu-south-1",
            "--tag-specifications", "ResourceType=instance,Tags=[{Key=Name,Value=}]", // add value
            "--query", "Instances[0].InstanceId",
            "--output", "text"
        ])
        .output()
        .expect("Failed to run AWS CLI");

    // 👇 Log stdout e stderr per capire eventuali errori
    println!("✅ EC2 creation output (stdout): {}", String::from_utf8_lossy(&output.stdout));
    println!("⚠️ EC2 creation error (stderr): {}", String::from_utf8_lossy(&output.stderr));

    let instance_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if instance_id.is_empty() {
        panic!("❌ Instance ID vuoto. Qualcosa è andato storto durante la creazione.");
    }

    let _ = Command::new("aws")
        .args([
            "ec2", "wait", "instance-running",
            "--instance-ids", &instance_id,
            "--region", "eu-south-1"
        ])
        .status()
        .expect("Failed to wait for instance");

    let ip_output = Command::new("aws")
        .args([
            "ec2", "describe-instances",
            "--instance-ids", &instance_id,
            "--query", "Reservations[0].Instances[0].PublicIpAddress",
            "--output", "text",
            "--region", "eu-south-1"
        ])
        .output()
        .expect("Failed to get public IP");

    let public_ip = String::from_utf8_lossy(&ip_output.stdout).trim().to_string();

    println!("✅ Istanza creata: {}", public_ip);

    InstanceInfo {
        instance_id,
        public_ip,
    }
}
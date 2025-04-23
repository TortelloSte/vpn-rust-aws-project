# VPN Rust Project

Automated setup of a WireGuard VPN on AWS EC2 using Rust.

## Requirements

- AWS CLI configured
- A valid SSH public key
- `wg-quick` and `WireGuard` installed locally
- IAM permissions to create EC2 instances
- Rust toolchain (`cargo`, `rustup`)

## What it does

1. Creates or reuses a t3.micro EC2 instance on AWS (is Free)
2. Automatically provisions the instance with WireGuard
3. Generates `client.conf` for you to connect
4. Allows secure browsing through the VPN

## Structure

```
├── src/
│   ├── aws.rs          # EC2 management
│   ├── config.rs       # Config
│   ├── main.rs         # Entrypoint
│   ├── ssh.rs          # SSH provisioning
│   └── wireguard.rs    # Key generation, wg setup
├── out/                # Generated keys and client config
│   ├── client.conf
│   ├── server_public.key
├── .env                # Environment variables (not committed)
├── .gitignore
└── README.md
```

## How to use

```bash
cargo run
```
After setup:
1. Find your WireGuard client configuration in `out/client.conf`
2. Import it into your WireGuard client
3. Activate the tunnel

## Test the connection

```bash
curl ifconfig.me     # Should return the EC2's public IP
```

### SSH into the EC2 Instance

You can manually connect to the EC2 instance for debugging or inspection:
```bash
ssh -i /path/to/your-key.pem ec2-user@<EC2_PUBLIC_IP>
```
Note: Ensure your SSH key is correctly associated with the instance. For Amazon Linux, the default user is ec2-user.

## Cleanup (Terminate the Instance)

To terminate and remove the EC2 instance:
```bash
aws ec2 terminate-instances --instance-ids <ID>
```
Get the instance ID with:
```bash
aws ec2 describe-instances --filters "Name=tag:Name,Values=RustVPN" --query "Reservations[*].Instances[*].InstanceId" --output text
```

## Troubleshooting
### Can't SSH into the instance?
1. Double-check the IP address and path to your .pem key file
2. Ensure the security group allows inbound connections on port 22

### WireGuard client won't connect?
1. Check if wg-quick is installed and working locally
2. Ensure the EC2 instance is running and wg0 is active
3. Verify that UDP port 51820 is open in the instance's security group

### client.conf not generated?
1. Make sure the out/ directory is writable
2. Look for errors in the Rust application output

### Instance doesn't show up in AWS console?
1. Try running aws ec2 describe-instances to list all active instances
2. Make sure you're in the correct AWS region
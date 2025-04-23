pub fn check_env() {
    // Loop through required environment variables
    for var in ["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "SSH_KEY_PATH"] {
        if std::env::var(var).is_err() {
            // Environment variable not set in .env file
            panic!("Variable '{}' not set in .env", var);
        }
    }
}
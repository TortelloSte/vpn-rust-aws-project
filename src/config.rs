pub fn check_env() {
    for var in ["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "SSH_KEY_PATH"] {
        if std::env::var(var).is_err() {
            panic!("⚠️  Variabile '{}' non impostata nel .env", var);
        }
    }
}
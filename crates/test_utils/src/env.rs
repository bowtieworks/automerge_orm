pub fn load_env() {
    dotenvy::from_filename(".env.test").ok();
    dotenvy::dotenv().ok();
}

use serde::Deserialize;

#[derive(Deserialize)]
struct Credentials {
    username: String,
    auth_type: String,
    auth_data: String,
}

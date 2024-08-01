use serde::Serialize;

#[derive(Serialize)]
pub struct DefaultResponse {
    message: String
}

impl DefaultResponse {

    pub fn new() -> Self {
        DefaultResponse {
            message: "Api Ok.".to_string()
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
pub struct CustomHttpError {
    code: i32,
    message: String
}

impl CustomHttpError {
    pub fn new(code: i32, message: String) -> Self {
        CustomHttpError { code, message }
    }
}
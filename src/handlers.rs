use actix_web::{HttpRequest, Responder, Result};

pub fn index(_req: HttpRequest) -> Result<String> {
    Ok(format!("Hello!"))
}

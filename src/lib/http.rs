use {
  actix_web::HttpResponse,
  serde::{Deserialize, Serialize}
};

pub mod lib {
  #[derive(Debug, Deserialize, Serialize)]
  pub struct NotFoundMessage {
    message: String,
  }

  impl NotFoundMessage {
    pub fn new(message: String) -> Self {
      Self {
        message
      }
    }
  }

  pub enum ResponseType<T> {
    Ok(T),
    NotFound(T),
    Created(T),
  }

  impl<T: Serialize> ResponseType {
    pub fn get_response(&self) -> HttpResponse {
      match self {
        ResponseType::Ok(payload) => HttpResponse::Ok()
          .content_type("application/json")
          .json(payload),
        ResponseType::NotFound(message) => HttpResponse::BadRequest()
          .content_type("application/json")
          .json(message),
        ResponseType::Created(payload) => HttpResponse::Created()
          .content_type("application/json")
          .json(payload),
      }
    }
  }
}
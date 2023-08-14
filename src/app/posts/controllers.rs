use actix_multipart::Multipart;
use actix_web::HttpResponse;

use crate::app::{errors::AppError, upload};

pub async fn create_sticker(payload: Multipart) -> actix_web::Result<HttpResponse, AppError> {
    let upload_status = upload::files::save_file_fs(payload).await;

    match upload_status {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("update_succeeded")),
        Err(e) => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("update_failed {e}"))),
    }
}

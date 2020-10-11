use std::io::Write;

use actix_multipart::Multipart;
use actix_identity::Identity;
use actix_web::{web, http, post, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};

#[post("/upload")]
pub async fn upload_img(mut payload: Multipart, id: Identity) -> Result<HttpResponse, Error> {
    match id.identity() {
        Some(_) => {
            // iterate over multipart stream
            while let Ok(Some(mut field)) = payload.try_next().await {
                let content_type = field.content_disposition().unwrap();
                let filename = content_type.get_filename().unwrap();
                let filepath = format!("./statics/img/{}", sanitize_filename::sanitize(&filename));

                // File::create is blocking operation, use threadpool
                let mut f = web::block(|| std::fs::File::create(filepath))
                    .await
                    .unwrap();

                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    // filesystem operations are blocking, we have to use threadpool
                    f = web::block(move || f.write_all(&data).map(|_| f)).await?;
                }
            }
            Ok(HttpResponse::Ok().into())
        }
        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish().await,
    }
}
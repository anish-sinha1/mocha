pub mod files {
    use std::error::Error;
    use std::io::Write;

    use actix_multipart::Multipart;
    use actix_web::web;
    use futures::{StreamExt, TryStreamExt};

    use crate::app::{dto::stickers::CreateSticker, errors::AppError, types::AssetBackend, util};

    pub async fn save_assets_fs(
        mut payload: Multipart,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let uploads = loop {
            let mut file_paths = Vec::<String>::new();
            if let Ok(Some(mut field)) = payload.try_next().await {
                // let file_name = field.content_disposition().get_filename();
                if let (Some(file_name), Some(friendly_name), Some(mime)) = (
                    field.content_disposition().get_filename(),
                    field.content_disposition().get_name(),
                    field.headers().get("content-type"),
                ) {}
            } else {
                break file_paths;
            };

            // match raw_field {
            //     Ok(maybe_field) => {
            //         match maybe_field {
            //             Some(field) => {
            //                 //
            //                 todo!()
            //             }
            //             None => break file_paths,
            //         }
            //     }
            //     Err(_) => todo!(),
            // };
        };
        Ok(uploads)
    }

    pub async fn save_file_fs(
        mut payload: Multipart,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(loop {
            if let Ok(Some(mut field)) = payload.try_next().await {
                let content_type = field.content_disposition();
                let file_name = content_type.get_filename().unwrap();
                let random_prefix = util::rng::random_string(8);
                let filepath = format!("./src/app/assets/{random_prefix}-{file_name}");

                log::debug!("{:#?}", field.headers().get("content-type").unwrap());
                log::debug!("{}", content_type.get_name().unwrap());
                log::debug!("{file_name}");
                let copy = filepath.clone();
                // let mut f = web::block(move || std::fs::File::create(copy)).await??;

                // while let Some(chunk) = field.next().await {
                //     match chunk {
                //         Ok(data) => f = web::block(move || f.write_all(&data).map(|_| f)).await??,
                //         Err(e) => {
                //             log::error!("error uploading image: {e}");
                //             Err(AppError::InternalServerError)?;
                //         }
                //     };
                // }
                break filepath;
            }
        })
    }

    pub async fn save_file(
        backend: AssetBackend,
        payload: Multipart,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        match backend {
            AssetBackend::Fs => save_file_fs(payload).await,
            AssetBackend::Aws => unimplemented!(),
            AssetBackend::Gcp => unimplemented!(),
            AssetBackend::Azure => unimplemented!(),
        }
    }
}

use rocket::http::ContentType;
use rocket::Data;

use rocket_multipart_form_data::mime;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};
use rocket::response::{Redirect};  
use image::load_from_memory_with_format;
use image::ImageFormat;

use s3::bucket::Bucket;
// use s3::S3Error;
use s3::creds::Credentials;
use s3::region::Region;

//TODO could do with a refactor
#[post("/<path>", data = "<data>")]
pub  fn upload(content_type: &ContentType, data: Data, path: String) -> Result<Redirect, &'static str> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    return Err("The file is too large.");
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err("The file is not an image.");
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("image");

    match image {
        Some(mut image) => {
            let raw = image.remove(0);

            let content_type = raw.content_type.unwrap();
            println!("content type: {}", content_type);
            let file_name = raw.file_name.unwrap_or("Image".to_string());
            let data = raw.raw;

            match load_from_memory_with_format(&data, ImageFormat::Png)
            {
                Ok(_) => 
                {
                    let access_key = &dotenv::var("ACCESS-KEY").unwrap();
                    let secret_key = &dotenv::var("SECRET-KEY").unwrap();
                    let bucket_name = "portfolio-lellouch";
                    let region: Region = "eu-west-2".parse().unwrap();
                    let credentials = Credentials::new_blocking(Some(access_key), Some(secret_key), None, None, None).unwrap();
                    let bucket = Bucket::new(bucket_name, region, credentials).unwrap();
                    let (_, code) = bucket.put_object_blocking(format!("{}/{}", path, file_name), &data, content_type.type_().as_str() ).unwrap();
                    println!("Upload: {}", code);
                    // let filepath = format!("assets/images/{}", file_name );
                    // std::fs::write(filepath, data).unwrap();
                }

                Err(_) => println!("Image not in correct format")
            }

            Ok(Redirect::to("/admin"))
        }
        None => Err("Please input a file."),
    }
}
use std::env;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::body::BoxBody;
use actix_web::web::Data;
use s3::creds::Credentials;
use s3::{Bucket, Region};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let endpoint = env::var("ENDPOINT").expect("ENDPOINT not found");
    let access_key = env::var("ACCESS_KEY").expect("ACCESS_KEY not found");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found");
    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found");
    
    let region = Region::Custom {
        region: "region".to_owned(),
        endpoint,
    };
    let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).expect("Credentials init error");
    let bucket = Bucket::new(&bucket_name, region, credentials).expect("Bucket init error").with_path_style();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(bucket.clone()))
            .default_service(web::route().to(default))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn default(req: HttpRequest, bucket: Data<Box<Bucket>>) -> impl Responder {
    if req.method() != "GET" || req.path() == "/" {
        return HttpResponse::NotFound().body("File not found");
    }
    
    let response_data = bucket.get_object(req.path()).await;
    match response_data { 
        Ok(response) => {
            HttpResponse::Ok().body(BoxBody::new(response.to_vec()))
        },
        Err(_) => {
            HttpResponse::NotFound().body("File not found")
        }
    }
}
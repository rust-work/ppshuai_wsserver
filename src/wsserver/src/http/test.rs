
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{client::{Client, Connector}, Error, HttpRequest, HttpResponse};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use openssl::ssl::{SslConnector, SslMethod};

pub async fn test(_req: HttpRequest) -> HttpResponse {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let now = std::time::Instant::now();
    let payload =
        client
        .get("https://upload.wikimedia.org/wikipedia/commons/f/ff/Pizigani_1367_Chart_10MB.jpg")
        .send()
        .await
        .unwrap()
        .body()
        .limit(20_000_000)  // sets max allowable payload size
        .await
        .unwrap();

    println!(
        "awc time elapsed while reading bytes into memory: {} ms",
        now.elapsed().as_millis()
    );

    HttpResponse::Ok().content_type("image/jpeg").body(payload)
}

pub async fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

pub async fn login(id: Identity) -> HttpResponse {
    id.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

pub fn doc_index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

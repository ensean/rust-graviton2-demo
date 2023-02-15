extern crate ec2_instance_metadata;
use serde::Serialize;
use actix_web::{get, web, Responder, Result};



#[derive(Serialize, Debug)]
pub struct RuntimeInfo {
    pub instance_id: String,
    pub instance_type: String,
    pub instance_az: String,
    pub rust_version: String,
}

impl std::fmt::Display for RuntimeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default)]
pub struct RuntimeInfoClient;

impl RuntimeInfoClient {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_runtime_info(&self) -> Result<RuntimeInfo, std::io::Error> {
        let client = ec2_instance_metadata::InstanceMetadataClient::new();
        let metadata = client.get().unwrap();
        let runtime_info = RuntimeInfo {
            instance_id: metadata.instance_id,
            instance_type: metadata.instance_type,
            instance_az: metadata.availability_zone,
            rust_version: "".to_string()
        };
        return Ok(runtime_info);
    }
}

// fn main() {
//     let client = RuntimeInfoClient::new();
//     let runtime_info = client.get_runtime_info().unwrap();
//     println!("{}", runtime_info);
// }

#[get("/")]
async fn index() -> Result<impl Responder> {
    let client = RuntimeInfoClient::new();
    let runtime_info = client.get_runtime_info().unwrap();
    Ok(web::Json(runtime_info))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
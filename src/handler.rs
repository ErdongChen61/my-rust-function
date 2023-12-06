use crate::config::HandlerConfig;
use actix_web::{http::Method, web::Data, HttpRequest, HttpResponse};
use log::info;
use std::env;
use std::fs;
use std::path::Path;
use serde::Deserialize;

fn list_dir_recursive(path: &Path) {
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        println!("File: {:?}", path.display());
                    } else if path.is_dir() {
                        println!("Directory: {:?}", path.display());
                        list_dir_recursive(&path);
                    }
                }
            }
        }
    }
}

fn list_dirs(path: &Path) {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            println!("Directory: {:?}", path.display());
                        } else {
                            println!("File: {:?}", path.display());   
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read directory {:?}: {}", path.display(), e);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    x: String,
}

// Implement your function's logic here
pub async fn index(req: HttpRequest, config: Data<HandlerConfig>) -> HttpResponse {
    info!("{:#?}", req);

    // Get the current directory
    match env::current_dir() {
        Ok(current_dir) => {
            println!("Current directory1: {:?}", current_dir);
            list_dir_recursive(&current_dir);
        }
        Err(e) => {
            eprintln!("Failed to get the current directory: {}", e);
        }
    }
    match env::var("x") {
        Ok(value) => println!("The value of x is: {}", value),
        Err(e) => println!("Couldn't read x ({})", e),
    }

    let yaml_str1 = std::fs::read_to_string("test/config.yaml").expect("Failed to read config file");
        
    // Deserialize the YAML string into your Config struct.
    let config1: Config = serde_yaml::from_str(&yaml_str1).expect("Failed to parse YAML");

    // Now you can access the configuration values as needed.
    println!("config1 {:?}", config1);

    let yaml_str2 = std::fs::read_to_string("bin/config.yaml").expect("Failed to read config file");
        
    // Deserialize the YAML string into your Config struct.
    let config2: Config = serde_yaml::from_str(&yaml_str2).expect("Failed to parse YAML");

    // Now you can access the configuration values as needed.
    println!("config2 {:?}", config2);
    

    if req.method() == Method::GET {
        HttpResponse::Ok().body(format!("Hello {}!\n", config.name))
    } else {
        HttpResponse::Ok().body(format!("Thanks {}!\n", config.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, http, test::TestRequest, web::Bytes};

    fn config() -> Data<HandlerConfig> {
        Data::new(HandlerConfig::default())
    }

    #[actix_rt::test]
    async fn get() {
        let req = TestRequest::get().to_http_request();
        let resp = index(req, config()).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(
            &Bytes::from(format!("Hello {}!\n", "world")),
            to_bytes(resp.into_body()).await.unwrap().as_ref()
        );
    }

    #[actix_rt::test]
    async fn post() {
        let req = TestRequest::post().to_http_request();
        let resp = index(req, config()).await;
        assert!(resp.status().is_success());
        assert_eq!(
            &Bytes::from(format!("Thanks {}!\n", "world")),
            to_bytes(resp.into_body()).await.unwrap().as_ref()
        );
    }
}

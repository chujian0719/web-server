use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
      let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
      let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
      let full_path = format!("{}/{}", public_path, file_name);
      let contents = fs::read_to_string(full_path);
      contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
  order_id: i32,
  order_date: String,
  order_status: String,
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
      let http::httprequest::Resource::Path(s) = &req.resource;
      let route: Vec<&str> = s.split("/").collect();

      match route[1] {
        "" => {
          HttpResponse::new("200", None, Self::load_file("index.html"))
        },
        "static" => {
          let path = &route[1..].join("/");
          match Self::load_file(path) {
            Some(contents) => {
              let mut map: HashMap<&str, &str> = HashMap::new();
              if s.ends_with(".css") {
                map.insert("Content-Type", "text/css");
              } else if s.ends_with(".js") {
                map.insert("Content-Type", "text/javascript");
              } else {
                map.insert("Content-Type", "text/html");
              }
              HttpResponse::new("200", Some(map), Some(contents))
            },
            None => {
              HttpResponse::new("404", None, Self::load_file("404.html"))
            }
          }
        },
        path => match Self::load_file(path) {
          Some(contents) => {
            HttpResponse::new("200", None, Some(contents))
          },
          None => {
            HttpResponse::new("404", None, Self::load_file("404.html"))
          }
        }          
      }
    }
}

impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
      HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl WebServiceHandler {
  fn load_json(file_name: &str) -> Vec<OrderStatus> {
    let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("DATA_PATH").unwrap_or(default_path);
    let full_path = format!("{}/{}{}", public_path, file_name, ".json");
    let json_contents = fs::read_to_string(full_path);
    let orders = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
    orders
  }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
      let httprequest::Resource::Path(s) = &req.resource;
      let route: Vec<&str> = s.split("/").collect();
      match route[2] {
        "shipping" if route.len() > 2 && route[3] == "orders" => {
          let body = serde_json::to_string(&Self::load_json(route[3])).unwrap();
          HttpResponse::new("200", None, Some(body))
        },
        _ => {
          HttpResponse::new("404", None, Self::load_file("404.html"))
        }
      }
    }
}
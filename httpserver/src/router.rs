
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use super::handler::{Handler, StaticPageHandler, PageNotFoundHandler, WebServiceHandler};
use std::io::prelude::*;
pub struct Router;

impl Router {
  pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
    match req.method {
      httprequest::Method::Get => match &req.resource {
        httprequest::Resource::Path(path) => {
          let route: Vec<&str> = path.split('/').collect();
          match route[1] {
            "api" => {
              let resp: HttpResponse = WebServiceHandler::handle(&req);
              let _ = resp.send_response(stream);
            },
            _ => {
              let resp: HttpResponse = StaticPageHandler::handle(&req);
              let _ = resp.send_response(stream);
            }
          }
        }
      },
      _ => {
        let resp: HttpResponse = PageNotFoundHandler::handle(&req);
        let _ = resp.send_response(stream);
      }
    }
  }
}
use http::httprequest::HttpRequest;

pub trait Handler {
  fn handle(req: &HttpRequest) -> HttpResponse;
  fn load_file() {

  }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;


impl Handler for StaticPageHandler {
  fn handle(req: &HttpRequest) -> HttpResponse {
    HttpResponse::new("200", None, Self::load_file("index.html"))
  }
}
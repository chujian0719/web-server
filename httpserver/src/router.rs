
use http::httprequest::HttpRequest;
pub struct Router;

impl Router {
  fn route(req: HttpRequest) -> () {
    match req.method {
      httprequest::Method::Get => match req.resource {
        httprequest::Resource::Path(path) => {
          let route: Vec<&str> = path.split('/').collect();
          match route[1] {
            
          }
        }
      },
    }
  }
}
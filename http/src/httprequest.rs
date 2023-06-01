use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
  fn from(s: &str) -> Self {
    match s {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Version  {
  V1_1,
  V2_0,
  Uninitialized,
}

impl From<&str> for Version {
  fn from(s: &str) -> Self {
    match s {
      "HTTP/1.1" => Version::V1_1,
      "HTTP/2.0" => Version::V2_0,
      _ => Version::Uninitialized,
    }
  }
}

#[derive(Debug)]
pub enum Resource {
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
  pub method: Method,
  pub version: Version,
  pub resource: Resource,
  pub headers: HashMap<String, String>,
}

impl From<String> for HttpRequest {
  fn from(req: String) -> Self {
    let mut parsed_method = Method::Uninitialized;
    let mut parsed_resource = Resource::Path(String::from(""));
    let mut parsed_version = Version::Uninitialized;
    let mut parsed_headers = HashMap::new();

    for line in req.lines() {
      if line.contains("HTTP") {
        let (method, resource, version) = process_req_line(line);
        parsed_method = method;
        parsed_resource = resource;
        parsed_version = version;
      }
    }

    HttpRequest {
      method: parsed_method,
      version: parsed_version,
      resource: parsed_resource,
      headers: parsed_headers,
    }
  }
}

 fn process_req_line(s:&str) -> (Method, Resource, Version) {
  let mut words = s.split_whitespace();

  let method = words.next().unwrap();
  let resource = words.next().unwrap();
  let version = words.next().unwrap();

  (
    method.into(),
    Resource::Path(resource.into()),
    version.into(),
  )
 }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_info() {
    let m:Method = "GET".into();
    assert_eq!(m, Method::Get);
  }

  #[test]
  fn test_read_http() {
    let s: String =
    String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n'");

    let req:HttpRequest = s.into();
  }
}
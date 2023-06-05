use std::collections::HashMap;
use std::io::{Result, Write};

#[derive (Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
  version: &'a str,
  status_code: &'a str,
  status_text: &'a str,
  headers: Option<HashMap<&'a str, &'a str>>,
  body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
  fn default() -> Self {
    Self {
      version: "HTTP/1.1",
      status_code: "200",
      status_text: "OK",
      headers: None,
      body: None,
    }
  }
}

impl<'a> From<HttpResponse<'a>> for String {
  fn from(res: HttpResponse) -> String {
    format!(
      "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
      res.version(),
      res.status_code(),
      res.status_text(),
      res.headers(),
      res.clone().body.unwrap().len(),
      res.body()
    )
  }
}

impl<'a> HttpResponse<'a> {
  pub fn new(
    status_code: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
  ) -> Self {
    let mut response: HttpResponse<'a> = HttpResponse::default();

    if status_code != "200" {
      response.status_code = status_code;
    }

    response.headers = match headers {
        Some(h) => Some(h),
        None => {
          let mut h = HashMap::new();
          h.insert("Content-Type", "text/html");
          Some(h)
        }
    };

    response.status_text = match response.status_code {
      "200" => "OK",
      "400" => "Bad Request",
      "404" => "Not Found",
      "500" => "Internal Server Error",
      _ => "Not Found",
    };

    response.body = match body {
      Some(b) => Some(b),
      None => Some("".into()),
    };

    response
  }


  pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
    let res_string: String = String::from(self.clone());
    let _ = write!(write_stream, "{}", res_string);
    Ok(())
  }

  fn version(&self) -> &str {
    self.version
  }

  fn status_code(&self) -> &str {
    self.status_code
  }

  fn status_text(&self) -> &str {
    self.status_text
  }

  fn headers(&self) -> String {
    let mut header_string:String = "".into();
    let header_map = self.headers.clone().unwrap();

    for (k,v) in header_map.iter() {
      header_string = format!("{}{}: {}\r\n", header_string, k, v);
    }

    header_string
  }

  fn body(&self) -> &str {
    match &self.body {
      Some(b) => b.as_str(),
      None => "",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_http_response_creation() {
    let s:String = HttpResponse::new("200", None, None).into();
    println!("{}", s);
  }
}




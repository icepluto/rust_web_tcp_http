use std::{
    collections::HashMap,
    io::{Result, Write},
};

#[derive(Debug, PartialEq, Clone)]
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
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse) -> String {
        let v = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length:{}\r\n\r\n{}",
            &v.version(),
            &v.status_code(),
            &v.status_text(),
            &v.headers(),
            &value.body.unwrap().len(),
            &v.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "bad request".into(),
            "404" => "not found".into(),
            "500" => "internet server error".into(),
            _ => "not found".into(),
        };
        response.body = body;
        response
    }
    //接受tcpstream作为参数
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
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
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_str: String = "".into();
        for (k, v) in map.iter() {
            header_str = format!("{}{}:{}\r\n", header_str, k, v);
        }
        header_str
    }
    pub fn body(&self) -> &str {
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
    fn test_httpresponse_new_200() {
        let status_test = HttpResponse::new("200", None, Some("hello".into()));
        let status_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("hello".into()),
        };
        assert_eq!(status_expected, status_test);
    }
    #[test]
    fn test_httpresponse_new_404() {
        let status_test = HttpResponse::new("404", None, Some("hello".into()));
        let status_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "not found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("hello".into()),
        };
        assert_eq!(status_expected, status_test);
    }

    #[test]
    fn test_httpresponse_tostring() {
        let status_expected = HttpResponse{
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "not found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("hi".into()),
        };
        let http_string:String = status_expected.into();
        let actual_string = "HTTP/1.1 404 not found\r\nContent-Type:text/html\r\nContent-Length:2\r\n\r\nhi";
        assert_eq!(http_string,actual_string);
    }
}

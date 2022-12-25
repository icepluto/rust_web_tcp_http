use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialization,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialization,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialization,
}

impl From<&str> for Version {
    fn from(value: &str) -> Version {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialization,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialization;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (k, v) = header_line(line);
                parsed_headers.insert(k, v);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn header_line(s: &str) -> (String, String) {
    let mut header_item = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_item.next() {
        key = k.to_string();
    }
    if let Some(v) = header_item.next() {
        value = v.to_string();
    }
    (key, value)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_method_into() {
        let s: Method = "GET".into();
        assert_eq!(s, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let s: Version = "HTTP/1.1".into();
        assert_eq!(s, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s:String = String::from("GET /greeting HTTP/1.1 \r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        headers_expected.insert("Accept".into(), " */*".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}

use std::collections::HashMap;

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
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // Read each line in the incoming HTTP request
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // If it is blank line, do nothing
            } else {
                parsed_msg_body = line;
            }
        }

        // Parse the incoming HTTP request into HttpRequest struct
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(req_line: &str) -> (Method, Resource, Version) {
    // Parse the request line into individual chunks split by white spaces
    let mut words = req_line.split_whitespace();
    // Extract request method from first line
    let method = words.next().unwrap();
    // Extract resource
    let resource = words.next().unwrap();
    // Extract version
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(header_line: &str) -> (String, String) {
    // Parse header line into words split by separator (':')
    let mut header_items = header_line.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    // Extract key
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    // Extract value
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(http_method: &str) -> Method {
        match http_method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(http_version: &str) -> Version {
        match http_version {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let method: Method = "GET".into();
        assert_eq!(method, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let http_version: Version = "HTTP/1.1".into();
        assert_eq!(http_version, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        /* http request example
        GET /greeting HTTP/1.1
        Host: localhost:3000
        User-Agent: curl/7.71.1
        Accept: *//*
        */
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}

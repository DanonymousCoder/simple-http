#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    route: Route,  // Ensure Route is defined somewhere
    version: Version,
    headers: HttpHeader,
    request_body: String
}

#[derive(Debug)]
struct HttpHeader {
    headers: HashMap<String, String>
}

impl HttpHeader {
    pub fn new(request: &str) -> Option<HttpHeader> {
        let httpheader = HttpHeader {
            headers: HashMap::new()
        };
        let (_, header_str) = request.split_once("\r\n")?;

        for line in header_str.split_terminator("\r\n") {
            if line.is_empty() {
                break;
            }
            let (header, value) = line.split_once(":") ?;
            httpheader
                .headers
                .insert(header.trim().to_string(), value.trim().to_string());
        }

        Some(httpheader)
    }
}

#[derive(Debug)]
enum Version {
    V1_1,
    V2_0,
}

#[derive(Debug)]
struct VersionError {
    msg: String
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Version {
    pub new()
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(request: &str) -> Result<Self, Self::Err> {
        let request_split  = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request {
            let method_line = method_line.split_ascii_whitespace();
            for line in method_line
            {
                if line = "HTTP/1.1" {
                    return Ok(Version::V1_1)
                } else if line == "HTTP/2" || line == "HTTP/2.0" {
                    return Ok(Version::V2_0);
                };
            };
        };
        let invalid = format!("Unknown protocol version in {}", request);
        let version_error = VersionError {
            msg:invalid
        };
        Err(version_error)
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Uninitialized
}

#[derive(Debug)]
struct Route {
    path: String,
}
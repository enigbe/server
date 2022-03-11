use std::str::FromStr;

/// HTTP methods
#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl FromStr for Method {
    type Err = MethodError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::GET),
            "DELETE" => Ok(Self::GET),
            "PUT" => Ok(Self::GET),
            "PATCH" => Ok(Self::GET),
            "HEAD" => Ok(Self::GET),
            "CONNECT" => Ok(Self::GET),
            "OPTIONS" => Ok(Self::GET),
            "TRACE" => Ok(Self::GET),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
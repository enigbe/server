use super::method::Method;

/// Request struct
pub struct  Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
pub mod context;
pub mod controllers;
pub mod db;
pub mod schemas;

use lambda_http::{
    Body, Request,
    http::{HeaderMap, HeaderValue, Method, Uri, header},
};
use serde_json::Value;

#[derive(Debug)]
pub struct CatchupContext {
    username: String,
}

#[derive(Debug)]
pub struct CatchupRequest<'a> {
    pub uri: &'a Uri,
    pub body: &'a Body,
    pub body_value: Value,
    pub headers: &'a HeaderMap<HeaderValue>,
    pub method: &'a Method,
    pub context: Option<CatchupContext>,
}

impl<'a> CatchupRequest<'a> {
    pub fn from(request: &'a Request) -> Self {
        let mut context = None;

        if let Some(cookies) = request.headers().get(header::COOKIE) {
            let cookies = cookie::Cookie::split_parse(cookies.to_str().unwrap());

            for cookie in cookies {
                let cookie = cookie.unwrap();

                if cookie.name() == "username" {
                    context = Some(CatchupContext {
                        username: cookie.value().to_string(),
                    });
                }
            }
        }

        Self {
            context,
            uri: request.uri(),
            body: request.body(),
            method: request.method(),
            headers: request.headers(),
            body_value: match request.body() {
                Body::Text(val) => {
                    serde_json::from_str(val).unwrap_or(Value::String(String::from(val)))
                }
                Body::Binary(val) => serde_json::json!(std::str::from_utf8(val).unwrap()),
                _ => {
                    serde_json::json!(null)
                }
            },
        }
    }
}

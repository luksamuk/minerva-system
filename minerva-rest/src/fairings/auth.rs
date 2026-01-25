//! This submodule contains the implementation of an authentication fairing.

use log::debug;
use minerva_data::session::Session;
use minerva_rpc as rpc;
use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_okapi::okapi::openapi3::{Object, SecurityRequirement, SecurityScheme};
use rocket_okapi::request::OpenApiFromRequest;
use std::env;

/// Struct for session information that can be retrieved for every access to a
/// route that explicitly retrieves it.
///
/// Adding a `State` containing a `SessionInfo` to a route function's parameters
/// automatically means that the route will only be accessible by an
/// authenticated user that passes its authentication info through cookies. If
/// a `SessionInfo` can be retrieved given the cookies information, then the
/// user is authenticated for using that route.
pub struct SessionInfo {
    /// Session info for the authenticated user.
    pub info: Session,
    /// Session token for the authenticated user.
    pub token: String,
}

impl SessionInfo {
    /// Generates a new session info from a `SessionData` gRPC message.
    pub fn from(info: rpc::messages::SessionData, token: String) -> Self {
        Self {
            info: info.into(),
            token,
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for SessionInfo {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires a Bearer Token to access.".to_owned()),
            data: rocket_okapi::okapi::openapi3::SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        Ok(rocket_okapi::request::RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}

/// Describes kinds of errors when attempting to retrieve a `SessionInfo`
/// through the workings of its fairing.
#[derive(Debug, Clone)]
pub enum SessionError {
    /// The cookies did not contain information on the tenant.
    MissingTenant,
    /// There is no authentication data for that user's session.
    MissingAuth,
    /// The authentication data has expired.
    ExpiredAuth,
    /// The required microservice could not be reached.
    ServiceUnreachable,
}

/// Extracts bearer token from authorization header, if any.
fn get_bearer_token(authorization: &str) -> Option<String> {
    let prefix = "Bearer ";
    if authorization.starts_with(prefix) {
        Some(authorization.trim_start_matches(prefix).to_owned())
    } else {
        None
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionInfo {
    type Error = SessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Strip API_ROOT from path
        let api_root = env::var("API_ROOT").unwrap_or_else(|_| String::new());
        let mut path = req.uri().path().to_string();
        if path.starts_with(&api_root) {
            path = path[api_root.len()..].to_owned();
        }
        let uri = Origin::parse(&path).expect("Stripped URI for auth is valid");

        debug!("Matched URI path (without api root): {}", uri);

        // Get tenant from first segment of uri, e.g. /<tenant>/...
        let tenant = match uri.path().segments().get(0) {
            Some(tenant) => tenant.to_owned(),
            None => return Outcome::Error((Status::Unauthorized, SessionError::MissingTenant)),
        };

        // Get auth token from request header
        match req.headers().get_one("Authorization") {
            None => Outcome::Error((Status::Unauthorized, SessionError::MissingAuth)),
            Some(header) => match get_bearer_token(header) {
                Some(token) => {
                    // Request a connection to SESSION service
                    let endpoint = crate::controller::auth::get_endpoint();
                    let msg = rpc::messages::SessionToken {
                        token: token.clone(),
                    };
                    let requestor = "unknown".into();

                    match rpc::session::make_client(endpoint, tenant, requestor).await {
                        Err(_) => {
                            return Outcome::Error((
                                Status::ServiceUnavailable,
                                SessionError::ServiceUnreachable,
                            ))
                        }
                        // Upon success, attempt to fetch session data
                        Ok(mut client) => match client.retrieve(tonic::Request::new(msg)).await {
                            Ok(response) => {
                                Outcome::Success(SessionInfo::from(response.into_inner(), token))
                            }
                            Err(_) => {
                                Outcome::Error((Status::Unauthorized, SessionError::ExpiredAuth))
                            }
                        },
                    }
                }
                None => Outcome::Error((Status::Unauthorized, SessionError::MissingAuth)),
            },
        }
    }
}

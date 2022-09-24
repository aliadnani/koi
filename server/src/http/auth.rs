use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::{header::AUTHORIZATION, StatusCode},
    Extension,
};

use crate::{
    sessions::{Session, SessionRepositoryDyn},
    users::{model::UserProfile, repo::UserRepositoryDyn},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthBearer(pub UserProfile);

#[async_trait]
impl<B> FromRequest<B> for AuthBearer
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(user_repo) = Extension::<UserRepositoryDyn>::from_request(req)
            .await
            .unwrap();

        let Extension(session_repo) = Extension::<SessionRepositoryDyn>::from_request(req)
            .await
            .unwrap();

        let authorization = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or((
                StatusCode::BAD_REQUEST,
                String::from("`Authorization` header is missing"),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    String::from("`Authorization` header contains invalid characters"),
                )
            })?;

        // Check that its a well-formed bearer and return
        let (_name, token) = match authorization.split_once(' ') {
            Some((name, contents)) if name == "Bearer" => (name, contents),
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    String::from("`Authorization` header must be a bearer token"),
                ))
            }
        };

        let email = match session_repo
            .verify_session(String::from(token))
            .await
            .unwrap()
        {
            Some(email) => email,
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    String::from("User with email does not exist"),
                ))
            }
        };

        let user_profile = user_repo
            .get_profile_by_email(String::from(email))
            .await
            .unwrap();

        user_profile.map(|u| Self(u)).ok_or((
            StatusCode::UNAUTHORIZED,
            String::from("User with email does not exist"),
        ))
    }
}

/// Decodes basic auth, returning the full tuple if present
fn decode_basic(input: &str) -> Result<(String, String), (StatusCode, &'static str)> {
    const ERR: (StatusCode, &'static str) = (
        StatusCode::BAD_REQUEST,
        "`Authorization` header's basic authentication was improperly encoded",
    );

    // Decode from base64 into a string
    let decoded = base64::decode(input).map_err(|_| ERR)?;
    let decoded = String::from_utf8(decoded).map_err(|_| ERR)?;

    // Return depending on if password is present
    if let Some((id, password)) = decoded.split_once(':') {
        Ok((id.to_string(), password.to_string()))
    } else {
        Err((StatusCode::BAD_REQUEST, "Password needed!"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthBasic(pub Session);

#[async_trait]
impl<B> FromRequest<B> for AuthBasic
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(user_repo) = Extension::<UserRepositoryDyn>::from_request(req)
            .await
            .unwrap();

        let Extension(session_repo) = Extension::<SessionRepositoryDyn>::from_request(req)
            .await
            .unwrap();

        let authorization = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or((
                StatusCode::BAD_REQUEST,
                String::from("`Authorization` header is missing"),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    String::from("`Authorization` header contains invalid characters"),
                )
            })?;

        let (email, password) = match authorization.split_once(' ') {
            Some((name, contents)) if name == "Basic" => decode_basic(contents).unwrap(),
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    String::from("`Authorization` header must be a bearer token"),
                ))
            }
        };

        let email = match user_repo
            .get_profile_by_email(String::from(email))
            .await
            .unwrap()
        {
            Some(user) => user.email,
            None => return Err((StatusCode::NOT_FOUND, String::from("User does not exist!"))),
        };
        // true => Ok(Self(session_repo.create_session(email.clone()).await.unwrap())),
        // false => Err((StatusCode::FORBIDDEN, String::from("Wrong credentials!"))),
        match user_repo
            .validate_profile_credentials(email.clone(), password)
            .await
            .unwrap()
        {
            Some(user_profile) => {
                let token = session_repo.create_session(email.clone()).await.unwrap();
                Ok(Self(Session {
                    token: token,
                    user_profile,
                }))
            }
            None => Err((StatusCode::FORBIDDEN, String::from("Wrong credentials!"))),
        }
    }
}

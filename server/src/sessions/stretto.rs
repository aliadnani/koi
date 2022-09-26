use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use eyre::{eyre, Result};

use nanoid::nanoid;
use stretto::AsyncCache;

use crate::common::ALPHANUMERIC;

use super::SessionRepository;

pub struct SessionRepositoryStretto {
    cache: Arc<AsyncCache<String, String>>,
}

impl SessionRepositoryStretto {
    pub fn new(cache: Arc<AsyncCache<String, String>>) -> SessionRepositoryStretto {
        SessionRepositoryStretto { cache }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryStretto {
    async fn verify_session(&self, token: String) -> Result<Option<String>> {
        match self.cache.get(&token) {
            Some(user_email) => Ok(Some(user_email.to_string())),
            None => Ok(None),
        }
    }

    async fn create_session(&self, user_email: String) -> Result<String> {
        let token = nanoid!(30, ALPHANUMERIC);

        match self
            .cache
            .insert_with_ttl(
                token.clone(),
                user_email,
                // I have no idea what cost is
                // need to look into https://github.com/dgraph-io/ristrettomore
                1,
                // Store user sessions for 7 days
                Duration::from_secs(7 * 24 * 3600),
            )
            .await
        {
            true => Ok(token),
            false => Err(eyre!("Could not set cache!")),
        }
    }
    async fn delete_session(&self, token: String) -> Result<()> {
        match self.cache.try_remove(&token).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

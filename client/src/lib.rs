mod action;
mod baseline;
pub mod contract;
mod engine;
mod link;
mod object;
mod realm;

use eyre::{eyre, Result, WrapErr};
use link::{Link, RealmServerHandle, Session};
use realm::{Realm, RealmID};

#[non_exhaustive]
pub enum Endpoint {
    Local(RealmServerHandle),
    Wss(String, u16),
    Tcp(String, u16),
}

// This could just be directly args on RealmClient::new(), but we expect building will
// be more complicated in the future.
pub struct RealmClientBuilder {
    endpoint: Endpoint,
    realm_id: String,
}
impl<'a> RealmClientBuilder {
    pub fn create(endpoint: Endpoint, realm_id: String) -> Self {
        Self { endpoint, realm_id }
    }

    pub fn build(self) -> Result<RealmClient<'a>> {
        RealmClient::new(self)
    }
}

/// Represents an API client for the realm.
pub struct RealmClient<'a> {
    session: Session,
    link: Link,
    local_realm: Realm<'a>,
}
impl<'a> RealmClient<'a> {
    fn new(builder: RealmClientBuilder) -> Result<Self> {
        let local_realm = Realm::new(RealmID::new(builder.realm_id));
        let result = match builder.endpoint {
            Endpoint::Local(handle) => {
                let link = Link::new(handle);
                let session = link.create_session();
                Self {
                    link,
                    session,
                    local_realm,
                }
            }
            _ => return Err(eyre!("We only support local endpoints!")),
        };
        Ok(result)
    }
}

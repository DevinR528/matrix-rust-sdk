use std::{fmt::Debug, sync::Arc};

use conduit::{
    client_server::join_room_by_id_route, ConduitResult, Config, Database, Error as ConduitError,
    Ruma, RumaResponse, State,
};
use http::Method as HttpMethod;
use reqwest::header::{HeaderValue, AUTHORIZATION};
#[cfg(not(target_arch = "wasm32"))]
use tokio::{
    runtime::Handle,
    sync::{
        mpsc::{channel, Receiver, Sender},
        RwLock,
    },
};
use url::Url;

use matrix_sdk_base::Session;
use matrix_sdk_common_macros::async_trait;

use crate::{ClientConfig, DefaultHttpClient, Error, HttpClient, Result};

/// A peer to peer enabled "http" client implementation.
#[derive(Clone, Debug)]
pub struct P2PClient {
    /// Incase we need to fallback or start a p2p session using http.
    inner: DefaultHttpClient,

    /// The other half of this is also in the second half of the conduit loop.
    from_conduit: Arc<RwLock<Receiver<reqwest::Response>>>,
}

impl P2PClient {
    /// Returns a `P2PClient` built with the default config.
    pub fn new(from_conduit: Receiver<reqwest::Response>) -> Self {
        Self::with_config(&ClientConfig::default(), from_conduit).unwrap()
    }

    /// Build a client with the specified configuration.
    pub fn with_config(
        config: &ClientConfig,
        from_conduit: Receiver<reqwest::Response>,
    ) -> Result<Self> {
        let http_client = DefaultHttpClient::with_config(config);
        let (snd, from_conduit) = channel(1024);

        Ok(Self {
            inner: http_client?,
            from_conduit: Arc::new(RwLock::new(from_conduit)),
        })
    }
}

#[async_trait]
impl HttpClient for P2PClient {
    async fn send_request(
        &self,
        _requires_auth: bool,
        _homeserver: &Url,
        _session: &Arc<RwLock<Option<Session>>>,
        _method: http::Method,
        _request: http::Request<Vec<u8>>,
    ) -> Result<reqwest::Response> {
        loop {
            match self.from_conduit.write().await.recv().await {
                Some(resp) => {
                    return Ok(reqwest::Response::from(resp));
                }
                None => {
                    // TODO do something productive
                    continue;
                }
            }
        }
    }
}

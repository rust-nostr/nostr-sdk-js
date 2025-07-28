// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;

use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

use crate::relay::limits::JsRelayLimits;

#[wasm_bindgen(js_name = ClientOptions)]
pub struct JsClientOptions {
    inner: ClientOptions,
}

impl Deref for JsClientOptions {
    type Target = ClientOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<ClientOptions> for JsClientOptions {
    fn from(inner: ClientOptions) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen(js_class = ClientOptions)]
impl JsClientOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: ClientOptions::new(),
        }
    }

    /// Automatically start connection with relays (default: false)
    ///
    /// When set to `true`, there isn't the need of calling the connect methods.
    pub fn autoconnect(self, val: bool) -> Self {
        self.inner.autoconnect(val).into()
    }

    /// Auto authenticate to relays (default: true)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/42.md>
    #[wasm_bindgen(js_name = automaticAuthentication)]
    pub fn automatic_authentication(self, enabled: bool) -> Self {
        self.inner.automatic_authentication(enabled).into()
    }

    /// Enable gossip model (default: false)
    pub fn gossip(self, enable: bool) -> Self {
        self.inner.gossip(enable).into()
    }

    /// Set custom relay limits
    #[wasm_bindgen(js_name = relayLimits)]
    pub fn relay_limits(self, limits: &JsRelayLimits) -> Self {
        self.inner.relay_limits(limits.deref().clone()).into()
    }
}

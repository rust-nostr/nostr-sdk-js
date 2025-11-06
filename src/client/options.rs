// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;

use nostr_sdk::prelude::*;
use wasm_bindgen::prelude::*;

use crate::relay::limits::JsRelayLimits;

/// Max number of relays to use for gossip
#[wasm_bindgen(js_name = GossipRelayLimits)]
pub struct JsGossipRelayLimits {
    /// Max number of **read** relays per user (default: 3)
    pub read_relays_per_user: usize,
    /// Max number of **write** relays per user (default: 3)
    pub write_relays_per_user: usize,
    /// Max number of **hint** relays per user (default: 1)
    pub hint_relays_per_user: usize,
    /// Max number of **most used** relays per user (default: 1)
    pub most_used_relays_per_user: usize,
    /// Max number of NIP-17 relays per user (default: 3)
    pub nip17_relays: usize,
}

impl From<JsGossipRelayLimits> for GossipRelayLimits {
    fn from(limits: JsGossipRelayLimits) -> Self {
        Self {
            read_relays_per_user: limits.read_relays_per_user,
            write_relays_per_user: limits.write_relays_per_user,
            hint_relays_per_user: limits.hint_relays_per_user,
            most_used_relays_per_user: limits.most_used_relays_per_user,
            nip17_relays: limits.nip17_relays,
        }
    }
}

/// Gossip options
#[wasm_bindgen(js_name = GossipOptions)]
pub struct JsGossipOptions {
    inner: GossipOptions,
}

impl From<GossipOptions> for JsGossipOptions {
    fn from(inner: GossipOptions) -> Self {
        Self { inner }
    }
}

impl Deref for JsGossipOptions {
    type Target = GossipOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[wasm_bindgen(js_class = GossipOptions)]
impl JsGossipOptions {
    /// Set limits
    #[inline]
    pub fn limits(self, limits: JsGossipRelayLimits) -> Self {
        self.inner.limits(limits.into()).into()
    }
}

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

    /// Set custom relay limits
    #[wasm_bindgen(js_name = relayLimits)]
    pub fn relay_limits(self, limits: &JsRelayLimits) -> Self {
        self.inner.relay_limits(limits.deref().clone()).into()
    }

    /// Set gossip options
    pub fn gossip(self, opts: &JsGossipOptions) -> Self {
        self.inner.gossip(**opts).into()
    }
}

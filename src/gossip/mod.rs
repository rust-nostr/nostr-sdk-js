use std::ops::Deref;
use std::sync::Arc;

use nostr_gossip::NostrGossip;
use nostr_gossip_memory::store::NostrGossipMemory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = NostrGossip)]
pub struct JsNostrGossip {
    inner: Arc<dyn NostrGossip>,
}

impl Deref for JsNostrGossip {
    type Target = Arc<dyn NostrGossip>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[wasm_bindgen(js_class = NostrGossip)]
impl JsNostrGossip {
    /// Construct a new in-memory gossip store
    #[wasm_bindgen(js_name = inmemory)]
    pub fn in_memory() -> Self {
        Self {
            inner: Arc::new(NostrGossipMemory::unbounded()),
        }
    }
}

const { Keys, Client, NostrSigner, NostrConnect, NostrConnectURI, loadWasmAsync, initLogger, LogLevel, Duration, EventBuilder } = require("../");

async function main() {
    await loadWasmAsync();

    initLogger(LogLevel.info());

    // App keys
    let appKeys = Keys.parse("nsec1lxxqxj4z2ru8p7d45m48thlcfaj594fw4jaj8tfp8zpxylh4ahsscs359t");

    // Remote signer (NIP46)
    let uri = NostrConnectURI.parse("bunker://...");
    let timeout = Duration.fromSecs(60);
    let connect = new NostrConnect(uri, appKeys, timeout);
    let signer = NostrSigner.nip46(connect);

    // Compose client and add relays
    let client = new Client(signer);
    await client.addRelay("wss://relay.damus.io");
    await client.addRelay("wss://nos.lol");
    await client.addRelay("wss://nostr.oxtr.dev");
    await client.connect();

    let builder = EventBuilder.textNote("Text note from rust-nostr WASM with NIP46 signer!");
    let output = await client.sendEventBuilder(builder);
    console.log("ID:", output.id.toBech32());
    console.log("Sent to:", output.success);
    console.log("Failed to send to:", output.failed);
}

main();

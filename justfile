set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

init:
	@wasm-pack --version || cargo install wasm-pack

# Check to perform before push a commit
precommit:
    @bash contrib/scripts/precommit.sh

# Execute a full checks
check:
    @bash contrib/scripts/check.sh

clean:
    rm -rf ./pkg
    rm -f rust-nostr-*.tgz
    cd benches && rm -rf node_modules package-lock.json

pack: init clean
	bash ./scripts/build.sh
	npm pack

[confirm]
publish: pack
	npm publish --access public

bench:
    cd benches && npm install && deno bench

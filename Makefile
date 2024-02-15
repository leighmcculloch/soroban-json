export SOROBAN_NETWORK=local
export SOROBAN_ACCOUNT=me

serve:
	cd web-app-demo && deno run --allow-net --allow-read https://deno.land/std/http/file_server.ts

build:
	@cd contract-json && soroban contract build --out-dir ../out
	@cd contract-json-alloc && soroban contract build --out-dir ../out

	soroban contract optimize --wasm ./out/json.wasm
	soroban contract optimize --wasm ./out/json_alloc.wasm

	ls -lah out/*.optimized.wasm

deploy:
	soroban contract deploy --wasm out/json.optimized.wasm
	soroban contract deploy --wasm out/json_alloc.optimized.wasm

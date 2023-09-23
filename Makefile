export SOROBAN_NETWORK=local
export SOROBAN_ACCOUNT=me

serve:
	cd web-app-demo && deno run --allow-net --allow-read https://deno.land/std/http/file_server.ts

build:
	@cd contract-json && soroban contract build --out-dir ../out

	soroban contract optimize --wasm ./out/json.wasm

	ls -lah out/*.optimized.wasm

deploy:
	soroban contract deploy --wasm out/json.optimized.wasm

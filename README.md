# Soroban JSON

| :warning: Code in this repo is demo material only. It has not been audited. Do not use to hold, protect, or secure anything. |
|-----------------------------------------|

This repo contains [Soroban] contracts that demonstrate how to decode JSON.

[Stellar]: https://stellar.org
[Soroban]: https://soroban.stellar.org

The contract in this repo accepts a bytes containing JSON, and extracts a single field out and returns it.

## Costs

Decoding a webauthn JSON and extracting one field uses the following resources.

When using a statically allocated buffer of sufficient size:
- WASM Size = 3,901 bytes
- CPU Instructions = 2,407,958

When using dynamically allocated memory:
- WASM Size = 8,270
- CPU Instructions = 4,278,990

## Example

The contracts within this repository both accept JSON and return the contents of the challenge field.

```
$ echo -n '{"type":"webauthn.get","challenge":"hJHFvaaoU7qkcH9kML46shLL_btpYGCA6ty3ie0M1Qw","origin":"http://localhost:4507","crossOrigin":false}' | od -A n -t x1 | sed 's/ *//g'
7b2274797065223a22776562617574686e2e676574222c226368616c6c656e6765223a22684a48467661616f5537716b6348396b4d4c343673684c4c5f62747059474341367479336965304d315177222c226f726967696e223a22687474703a2f2f6c6f63616c686f73743a34353037222c2263726f73734f726967696e223a66616c73657d

❯ soroban -v contract invoke --network local --source me --id CD3WRM5QRWDCKGZBOEOYINLAHMLZGFDEJCDJVVWJHP6QLMA2AKQAYDSD -- extract --json 7b2274797065223a22776562617574686e2e676574222c226368616c6c656e6765223a22684a48467661616f5537716b6348396b4d4c343673684c4c5f62747059474341367479336965304d315177222c226f726967696e223a22687474703a2f2f6c6f63616c686f73743a34353037222c2263726f73734f726967696e223a66616c73657d
hJHFvaaoU7qkcH9kML46shLL_btpYGCA6ty3ie0M1Qw
```

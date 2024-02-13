# Soroban JSON

| :warning: Code in this repo is demo material only. It has not been audited. Do not use to hold, protect, or secure anything. |
|-----------------------------------------|

This repo contains [Soroban] contracts that demonstrate how to decode JSON.

[Stellar]: https://stellar.org
[Soroban]: https://soroban.stellar.org

The contract in this repo accepts a bytes containing JSON, and extracts a single field out and returns it.

For example:

```
$ echo -n '{"field":"ohno"}' | od -A n -t x1 | sed 's/ *//g'
7b226669656c64223a226f686e6f227d

$ soroban contract invoke --source me --network local \
    --id CC7LGJYXOM5P3MRG3BP4NWZKLCZQBVKN4TDTKXYBEFQCFKDDZOY5PQG2 \
    -- \
    extract \
    --data 7b226669656c64223a226f686e6f227d
"ohno"

$ soroban -v contract invoke --source me --network local \
    --id CC7LGJYXOM5P3MRG3BP4NWZKLCZQBVKN4TDTKXYBEFQCFKDDZOY5PQG2 \
    -- \
    extract \
    --data 7b226669656c64223a226f686e6f227d
2024-02-13T12:13:27.437496Z DEBUG soroban_cli::log::cost: cost===================== Cost ====================
CPU used: 5040410
Bytes read: 3956
Bytes written: 0
==============================================

2024-02-13T12:13:27.437950Z DEBUG soroban_cli::log::auth: [
    VecM(
        [],
    ),
]
2024-02-13T12:13:27.437964Z DEBUG soroban_cli::log::footprint: LedgerFootprint {
    read_only: VecM(
        [
            ContractData(
                LedgerKeyContractData {
                    contract: Contract(
                        Hash(beb32717733afdb226d85fc6db2a58b300d54de4c7355f01216022a863cbb1d7),
                    ),
                    key: LedgerKeyContractInstance,
                    durability: Persistent,
                },
            ),
            ContractCode(
                LedgerKeyContractCode {
                    hash: Hash(b3ec234016bf3ca937fcb81cb86a0d0d44552643e3de93b599cbd363481c4804),
                },
            ),
        ],
    ),
    read_write: VecM(
        [],
    ),
}
2024-02-13T12:13:28.463768Z DEBUG soroban_cli::commands::contract::invoke: result=TransactionResult { fee_charged: 59729, result: TxSuccess(VecM([OpInner(InvokeHostFunction(Success(Hash(3fe6d1046ef38893001c5c13f39d63b2e205b0c7ae8b4f7c2ff87389d36f80ee))))])), ext: V0 }
"ohno"
```

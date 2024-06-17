# Fuels bug when incrementing time

Fuelup version
```
Default host: aarch64-apple-darwin
fuelup home: /Users/lucaauet/.fuelup

installed toolchains
--------------------
latest-aarch64-apple-darwin (default)

active toolchain
----------------
latest-aarch64-apple-darwin (default)
  forc : 0.60.0
    - forc-client
      - forc-deploy : 0.60.0
      - forc-run : 0.60.0
    - forc-crypto : 0.60.0
    - forc-debug : 0.60.0
    - forc-doc : 0.60.0
    - forc-explore : 0.28.1
    - forc-fmt : 0.60.0
    - forc-lsp : 0.60.0
    - forc-tx : 0.60.0
    - forc-wallet : 0.7.1
  fuel-core : 0.26.0
  fuel-core-keygen : 0.26.0

fuels versions
--------------
forc : 0.62.0
forc-wallet : 0.62.0
```

To reproduce, first compile the contract in debug mode:
```bash
forc build
```

And then run the tests:
```bash
cargo test
```

Two tests will end successfully, but the third one will fail. The test that fails returns the following revert error:
```bash
Err(Transaction(Reverted { reason: "OutOfGas", revert_id: 0, receipts: [Call { id: 0000000000000000000000000000000000000000000000000000000000000000, to: c25cde1a75492ee75fa2f558b275a5db79582f22edcd63e5d838819627a07b5b, amount: 1, asset_id: 0000000000000000000000000000000000000000000000000000000000000000, gas: 1213, param1: 10480, param2: 10507, pc: 11688, is: 11688 }, Panic { id: c25cde1a75492ee75fa2f558b275a5db79582f22edcd63e5d838819627a07b5b, reason: PanicInstruction { reason: OutOfGas, instruction: ADDI { dst: 0x11, lhs: 0x3b, rhs: 408 } (bytes: 50 47 b1 98) }, pc: 15324, is: 11688, contract_id: None }, ScriptResult { result: Panic, gas_used: 1388 }] }))
```

# meta-tx

# Overview

ink! Meta transaction contract implementations.
Contracts are implemented based on the [ERC2771](https://eips.ethereum.org/EIPS/eip-2771) specifications.
Corresponding solidity implementations can be found [here](https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/metatx).

`Forwarder` contract - Forwarder contracts verify signers' signature and forward transaction requests to actual callee contracts.
`MetaTxContext` crate - Callee contracts which want to make use of signers' information need to implements `MetaTxContext` trait.
`Registry` contract - An example contract implementing `MetaTxContext`.

# How to test
1. Run local node e.g. [swanky-node](https://github.com/AstarNetwork/swanky-node)
2. Install dependencies. `yarn`.
3. Test. `yarn test`.

Original forwarder contract implementation with no MetaTxContract can be found [here](https://github.com/jakerumbles/ink-meta-transaction). The Forwarder contract in this reposiroty is referencing to original implementation, and small modifications were added to it.

# How to use `MetaTxContext`

1. Import `meta-tx-context` crate.
2. Use crate and implement `MetaTxContext` trait.
```
use meta_tx_context::*;

impl MetaTxContext for Contract {}
```

3. Take Uint8 Vec `Vec<u8>` as the last argment of messages.
For example,
```
pub fn register(&mut self, name: String, data: Vec<u8>) -> Result<(), Error> {}
```
This is needed to decode call inputs which cotains Signer's AccountId as the extra 32bytes attached by Forwarder contract.

4. Use `self._caller(data)` instead of `self.env().caller()`.

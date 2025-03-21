# Ejit-EVM

An example of using EJIT on the blockchain.

## Overview

The ethererum EVM is a well documented IR format which is used to execute smart
contracts such as ERC20 tokens on blockchains.

We can improve the performance of the EVM considerably by translating the bytecode
into Ejit portable assembler.

See [https://github.com/ethereum/execution-specs](https://github.com/ethereum/execution-specs) for
details of the specificiation.

See [https://github.com/ethereum/tests](https://github.com/ethereum/tests) for tests.

See [https://github.com/bluealloy/revm/tree/main](https://github.com/bluealloy/revm/tree/main) for the
current best-in-class EVM.

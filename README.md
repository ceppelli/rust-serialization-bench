# SOLANA Goggip TUI
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.65+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://github.com/ceppelli/rust-serialization-bench/actions/workflows/rust.yml/badge.svg)](https://github.com/ceppelli/rust-serialization-bench/actions/workflows/rust.yml)


![Rust By Ceppelli Logo](assets/rust-by-ceppelli-128x128.png)


### Bench the Solana Gossip Protocol.

Performance comparison between serialization and deserialization libraries for the Solana Gossip Protocol.

---

## Forewords

The [Solana Blockchain](https://solana.com) is one of the most popular blockchains in the world. Its success is mainly due to its speed, with approximately 3,400 transactions per second, compared to 15 transactions per second for Ethereum, making Solana one of the fastest blockchain networks currently available.

The Gossip Service is the base-ground of the entire network. Every tenth of a second each network’s Node share signed data objects among themselves in order to manage a cluster.  The validator's services listener configuration, contract information, ledger height and vote are some example of exchanged data.

---

## Motivation

In the last weeks, in my spare time, I have been working on this project [Solana Gossip TUI](https://github.com/ceppelli/solana-gossip-tui) and today Sunday 23 April 2023 searching for comparisons between serialization and deserialization libraries, I came across the following repo [rust_serialization_benchmark](https://github.com/djkoloski/rust_serialization_benchmark).


At this point a question came to my mind: is there another library than the one used by **Solana Gossip Service** that could improve its performance?

The **Solana Gossip Service** currently use [bincode](https://crates.io/crates/bincode) and this are the other I want to give a try.


| Library            | Crate |
| :---               | :---:  |
| serde_son        | [🎯](https://crates.io/crates/serde_json) |
| bincode        | [🎯](https://crates.io/crates/bincode) |
| alkahest        | [🎯](https://crates.io/crates/alkahest) |
| rkyv        | [🎯](https://crates.io/crates/rkyv) |


## ⚠️ This project is deeply inspired by the [rust_serialization_benchmark](https://github.com/djkoloski/rust_serialization_benchmark) project. Thanks [djkoloski](https://github.com/djkoloski) for your work!!!  ⚠️
---

## Usage


```
cargo bench
```

---

## Results

### Ping Message

| Library     | Serialize | Deserialize |
| :---        |  ---:     |  ---:      |
| serde_son   | 548.98 ns | 1016.20 ns |
| bincode     | 202.46 ns |   63.98 ns |
| alkahest    |  66.48 ns |   35.08 ns |
| rkyv       |   6.94 ns |   26.75 ns |
| rkyv unsafe |         |   24.36 ns |


### Pong Message

| Library     | Serialize | Deserialize |
| :---        |  ---:     |  ---:      |
| serde_son   | 564.49 ns | 1016.20 ns |
| bincode     | 203.54 ns |   64.21 ns |
| alkahest    |  67.06 ns |   35.48 ns |
| rkyv       |   7.04 ns |  26.78 ns |
| rkyv unsafe |         |  24.379 ns |

### Pull Request Message

| Library     | Serialize | Deserialize |
| :---        |  ---:     |  ---:      |
| serde_son   | 1442.40 ns | 2589.80 ns |
| bincode     | 355.79 ns |   171.78 ns |
| alkahest    |  152.75 ns |  146.57 ns |
| rkyv       |  34.47 ns |   62.15 ns |
| rkyv unsafe |         |   53.45 ns |

### Pull Response Message

| Library     | Serialize  | Deserialize |
| :---        |  ---:      |  ---:      |
| serde_son   | 10831.00 ns | 23099.00 ns |
| bincode     | 3649.90 ns | 3284.20 ns |
| alkahest    | 3075.60 ns | 3280.10 ns |
| rkyv       |  816.79 ns | 2106.40 ns |
| rkyv unsafe |         |  1982.70 ns |

---

## Conglusion

As it is easy to understand there are tangible benefits in terms of performance to use different libraries.

This wasn't meant to be a rigorous test, but rather an answer to my curiosity.


---

## Clarifications

To make use of all these serialization libraries, some of the types used in the protocol structures have been implemented in a dummy way, just paying attention to respect the size of the underlying serialized values.

So for example the **std::net::SocketAddr** has been replaced with

```rust
pub struct SocketAddr {
    ip: [u8; 4],
    port: u16,
}

```

or the **bv::BitVec** has been replaced with

```rust
pub struct BitVec<Block> {
    bits: Option<Block>,
    len: u64,
}

```

In this way, besides being able to make the test work, it was also possible to avoid external dependencies.

--

## License

Copyright 2023 Luca Ceppelli

Licensed under the MIT license
<LICENSE-MIT or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)>. Files in the project may not be
copied, modified, or distributed except according to those terms.
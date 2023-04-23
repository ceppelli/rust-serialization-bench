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

In the last weeks, in my spare time, I have been working on this project [Solana Gossip TUI](https://github.com/ceppelli/solana-gossip-tui) and today Sunday 23 April 2023 searching for comparisons between serialization and deserialization libraries, I came accross the following repo [serializtion benchmark](https://github.com/djkoloski/rust_serialization_benchmark).


At this point a question came to my mind: is there another library than the one used by **Solana Gossip Service** that could improve its performance?

The **Solana Gossip Service** currently use [bincode](https://crates.io/crates/bincode) and this are the other I want to give a try.


| Library            | Crate |
| :---               | :---:  |
| serde_son        | [🎯](https://crates.io/crates/serde_json) |
| bincode        | [🎯](https://crates.io/crates/bincode) |
| alkahest        | [🎯](https://crates.io/crates/alkahest) |
| rkyv        | [🎯](https://crates.io/crates/rkyv) |



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
| rkyv       |  6.943 ns |    6.37 ns |
| rkyv unsafe |         |    1.02 ns |


### Pong Message

| Library     | Serialize | Deserialize |
| :---        |  ---:     |  ---:      |
| serde_son   | 564.49 ns | 1016.20 ns |
| bincode     | 203.54 ns |   64.21 ns |
| alkahest    |  67.06 ns |   35.48 ns |
| rkyv       |   7.04 ns |    6.35 ns |
| rkyv unsafe |         |    1.02 ns |

### Pull Request Message

| Library     | Serialize | Deserialize |
| :---        |  ---:     |  ---:      |
| serde_son   | 1442.40 ns | 2589.80 ns |
| bincode     | 355.79 ns |   171.78 ns |
| alkahest    |  152.75 ns |  146.57 ns |
| rkyv       |  34.47 ns |   14.44 ns |
| rkyv unsafe |         |    1.02 ns |

### Pull Response Message

| Library     | Serialize  | Deserialize |
| :---        |  ---:      |  ---:      |
| serde_son   | 10831.00 ns | 23099.00 ns |
| bincode     | 3649.90 ns | 3284.20 ns |
| alkahest    | 3075.60 ns | 3280.10 ns |
| rkyv       |  816.79 ns |  123.02 ns |
| rkyv unsafe |         |    1.02 ns |

---

## Clarifications

--

## License

Copyright 2023 Luca Ceppelli

Licensed under the MIT license
<LICENSE-MIT or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)>. Files in the project may not be
copied, modified, or distributed except according to those terms.
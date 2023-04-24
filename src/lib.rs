#[cfg(feature = "alkahest")]
pub mod bench_alkahest;
#[cfg(feature = "bincode")]
pub mod bench_bincode;
#[cfg(feature = "rkyv")]
pub mod bench_rkyv;
#[cfg(feature = "serde_json")]
pub mod bench_serde_json;

pub mod solana_gossip_proto;

pub fn bench_size(name: &str, lib: &str, bytes: &[u8]) {
    println!("{}/{}/size {}", name, lib, bytes.len());
}

use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "alkahest")]
use rust_serialization_bench::bench_alkahest;
#[cfg(feature = "bincode")]
use rust_serialization_bench::bench_bincode;
#[cfg(feature = "rkyv")]
use rust_serialization_bench::bench_rkyv;
#[cfg(feature = "serde_json")]
use rust_serialization_bench::bench_serde_json;

use rust_serialization_bench::solana_gossip_proto::{
    crate_ping, crate_pong, crate_pull_request_legacy_contact_info,
    crate_pull_response_legacy_contact_info, Protocol,
};

fn bench_ping(c: &mut Criterion) {
    const BENCH: &'static str = "ping";

    let data = crate_ping();

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "alkahest")]
    bench_alkahest::bench::<Protocol, Protocol>(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(BENCH, c, &data);
}

fn bench_pong(c: &mut Criterion) {
    const BENCH: &'static str = "pong";

    let data = crate_pong();

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "alkahest")]
    bench_alkahest::bench::<Protocol, Protocol>(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(BENCH, c, &data);
}

fn bench_crate_pull_request_legacy_contact_info(c: &mut Criterion) {
    const BENCH: &'static str = "pull_request_legacy_contact_info";

    let data = crate_pull_request_legacy_contact_info();

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "alkahest")]
    bench_alkahest::bench::<Protocol, Protocol>(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(BENCH, c, &data);
}

fn bench_crate_pull_response_legacy_contact_info(c: &mut Criterion) {
    const BENCH: &'static str = "pull_response_legacy_contact_info";

    let data = crate_pull_response_legacy_contact_info();

    #[cfg(feature = "serde_json")]
    bench_serde_json::bench(BENCH, c, &data);

    #[cfg(feature = "bincode")]
    bench_bincode::bench(BENCH, c, &data);

    #[cfg(feature = "alkahest")]
    bench_alkahest::bench::<Protocol, Protocol>(BENCH, c, &data);

    #[cfg(feature = "rkyv")]
    bench_rkyv::bench(BENCH, c, &data);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_ping(c);
    bench_pong(c);
    bench_crate_pull_request_legacy_contact_info(c);
    bench_crate_pull_response_legacy_contact_info(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

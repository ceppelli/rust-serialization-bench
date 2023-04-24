use criterion::{black_box, Criterion};

use alkahest::{deserialize, serialize, Deserialize, Formula, Serialize};

pub fn bench<'a, F, T>(name: &'static str, c: &mut Criterion, data: &T)
where
    F: Formula + ?Sized,
    T: Serialize<F> + for<'de> Deserialize<'de, F> + Clone,
{
    const BUFFER_LEN: usize = 1_000_000;

    let mut group = c.benchmark_group(format!("{name}/alkahest"));

    let mut buffer = vec![0; BUFFER_LEN];
    let mut size = 0;

    group.bench_function("serialize", |b| {
        b.iter(|| {
            size = serialize::<F, T>(black_box(data.clone()), &mut buffer.as_mut_slice()).unwrap();
            black_box(());
        });
    });

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(deserialize::<F, T>(&buffer[..size]).unwrap());
        });
    });

    crate::bench_size(name, "alkahest", &buffer[..size]);

    group.finish();
}

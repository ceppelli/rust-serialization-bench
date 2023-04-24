use bytecheck::CheckBytes;
use criterion::{black_box, Criterion};
use rkyv::{
    archived_value, check_archived_value,
    ser::{
        serializers::{AlignedSerializer, BufferScratch, CompositeSerializer},
        Serializer,
    },
    validation::validators::DefaultValidator,
    AlignedVec, Archive, Deserialize, Infallible, Serialize,
};

pub type BenchSerializer<'a> = CompositeSerializer<
    AlignedSerializer<&'a mut AlignedVec>,
    BufferScratch<&'a mut AlignedVec>,
    Infallible,
>;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Archive + for<'a> Serialize<BenchSerializer<'a>>,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, Infallible>,
{
    const BUFFER_LEN: usize = 1_000_000;
    const SCRATCH_LEN: usize = 512_000;

    let mut group = c.benchmark_group(format!("{name}/rkyv"));

    let mut serialize_buffer = AlignedVec::with_capacity(BUFFER_LEN);
    let mut serialize_scratch = AlignedVec::with_capacity(SCRATCH_LEN);
    unsafe {
        serialize_scratch.set_len(SCRATCH_LEN);
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            serialize_buffer.clear();

            let mut serializer = CompositeSerializer::new(
                AlignedSerializer::new(black_box(&mut serialize_buffer)),
                BufferScratch::new(black_box(&mut serialize_scratch)),
                Infallible,
            );
            black_box(serializer.serialize_value(black_box(data)).unwrap());
        });
    });

    serialize_buffer.clear();
    let mut serializer = CompositeSerializer::new(
        AlignedSerializer::new(&mut serialize_buffer),
        BufferScratch::new(&mut serialize_scratch),
        Infallible,
    );
    let pos = serializer.serialize_value(data).unwrap();
    let deserialize_buffer = serializer.into_serializer().into_inner();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let value =
                check_archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
                    .unwrap();
            let deserialized: T = value.deserialize(&mut Infallible).unwrap();
            black_box(deserialized);
        });
    });

    group.bench_function("deserialize (unsafe max performance)", |b| {
        b.iter(|| {
            let value = unsafe {
                archived_value::<T>(black_box(deserialize_buffer.as_ref()), black_box(pos))
            };
            let deserialized: T = value.deserialize(&mut Infallible).unwrap();
            black_box(deserialized);
        });
    });

    crate::bench_size(name, "rkyv", deserialize_buffer);

    group.finish();
}

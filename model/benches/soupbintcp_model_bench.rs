use byteserde::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use links_core::unittest::setup;
use soupbintcp_model::prelude::*;

fn soupbintcp_seq(c: &mut Criterion) {
    const MAX_FRAME_SIZE: usize = 1024 * 10;

    let msg_inp = SPayload::new(VecPayload::new(setup::data::random_bytes(MAX_FRAME_SIZE - 3).to_vec()));
    c.bench_function("soupbintcp_seq_vec_payload_ser", |b| {
        b.iter(|| {
            black_box({
                let _: ([u8; MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
            })
        })
    });

    let (buf, size): ([u8; MAX_FRAME_SIZE], usize) = to_bytes_stack(&msg_inp).unwrap();
    c.bench_function("soupbintcp_seq_vec_payload_des", |b| {
        b.iter(|| {
            black_box({
                let _: SPayload<VecPayload> = from_slice(&buf[..size]).unwrap();
            })
        })
    });
}

criterion_group!(benches, soupbintcp_seq);
criterion_main!(benches);

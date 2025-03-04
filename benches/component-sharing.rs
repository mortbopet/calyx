use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion,
};

use calyx::{frontend, ir, passes};
use ir::traversal::Visitor;
use std::path::Path;

fn resource_sharing_bench(c: &mut Criterion) {
    let mut gemm_group = c.benchmark_group("gemm");
    for name in &["gemm2", "gemm3", "gemm4", "gemm6", "gemm8"] {
        gemm_group.bench_with_input(
            BenchmarkId::from_parameter(name),
            name,
            |b, &name| {
                b.iter_batched(
                    || {
                        let name =
                            format!("benches/component-sharing/{}.futil", name);
                        let bench = Path::new(&name);
                        let lib = Path::new(".");

                        let ws = frontend::Workspace::construct(
                            &Some(bench.into()),
                            lib,
                        )
                        .unwrap();

                        ir::from_ast::ast_to_ir(ws, false, true).unwrap()
                    },
                    |mut rep: ir::Context| {
                        passes::ResourceSharing::do_pass_default(&mut rep)
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
    gemm_group.finish();
}

criterion_group! {
    name = resource_sharing;
    config = Criterion::default().sample_size(20);
    targets = resource_sharing_bench
}
criterion_main!(resource_sharing);

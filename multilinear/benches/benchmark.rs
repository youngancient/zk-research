use ark_bn254::Fq;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multilinear::evaluation_form::{find_pairs_with_xor, EvaluationForm};

fn benchmark(c: &mut Criterion) -> () {
    let mut group = c.benchmark_group("multilinear");
    let mut poly = EvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);

    let mut poly_of_3vars = EvaluationForm::new(vec![
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
        Fq::from(0),
        Fq::from(0),
        Fq::from(2),
        Fq::from(5),
    ]);

    let a_target = 2;

    group.bench_function("find_pairs_with_xor", |b| {
        b.iter(|| black_box(find_pairs_with_xor(&poly.boolean_hypercube, a_target)))
    });
    group.bench_function("find_pairs_with_xor_3vars", |b| {
        b.iter(|| {
            black_box(find_pairs_with_xor(
                &poly_of_3vars.boolean_hypercube,
                a_target,
            ))
        })
    });
    group.bench_function("partial_evaluate", |b| {
        b.iter(|| black_box(poly.partial_evaluate(a_target, Fq::from(2))));
    });
    group.bench_function("partial_evaluate_3vars", |b| {
        b.iter(|| black_box(poly_of_3vars.partial_evaluate(a_target, Fq::from(2))));
    });
    group.bench_function("evaluate", |b| {
        b.iter(|| black_box(poly.evaluate(vec![Fq::from(2), Fq::from(3)])))
    });
    group.bench_function("evaluate", |b| {
        b.iter(|| black_box(poly_of_3vars.evaluate(vec![Fq::from(4), Fq::from(2), Fq::from(3)])))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

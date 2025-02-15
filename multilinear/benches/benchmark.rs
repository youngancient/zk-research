use ark_bn254::Fq;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multilinear::evaluation_form::{
    find_pairs_with_xor, gen_based_on_two, gen_random_vars, MultilinearEvalForm,
};

fn benchmark(c: &mut Criterion) -> () {
    let mut group = c.benchmark_group("multilinear");
    let poly = MultilinearEvalForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);
    let poly_of_3vars = MultilinearEvalForm::new(vec![
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
        Fq::from(0),
        Fq::from(0),
        Fq::from(2),
        Fq::from(5),
    ]);

    let poly_of_10vars = MultilinearEvalForm::<Fq>::new(gen_based_on_two(10));
    // let ten_vars:Vec<Fq> = gen_random_vars(10);

    let poly_of_20vars = MultilinearEvalForm::<Fq>::new(gen_based_on_two(20));
    // let twenty_vars:Vec<Fq> = gen_random_vars(20);

    let a_target = 2;

    // find pairs
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
    group.bench_function("find_pairs_with_xor_10vars", |b| {
        b.iter(|| {
            black_box(find_pairs_with_xor(
                &poly_of_10vars.boolean_hypercube,
                a_target,
            ))
        })
    });
    group.bench_function("find_pairs_with_xor_20vars", |b| {
        b.iter(|| {
            black_box(find_pairs_with_xor(
                &poly_of_20vars.boolean_hypercube,
                a_target,
            ))
        })
    });

    // partial evaluate
    group.bench_function("partial_evaluate", |b| {
        b.iter(|| black_box(poly.clone().partial_evaluate(a_target, Fq::from(2))));
    });
    group.bench_function("partial_evaluate_3vars", |b| {
        b.iter(|| {
            black_box(
                poly_of_3vars
                    .clone()
                    .partial_evaluate(a_target, Fq::from(2)),
            )
        });
    });
    group.bench_function("partial_evaluate_10vars", |b| {
        b.iter(|| {
            black_box(
                poly_of_10vars
                    .clone()
                    .partial_evaluate(a_target, Fq::from(2)),
            )
        });
    });
    group.bench_function("partial_evaluate_20vars", |b| {
        b.iter(|| {
            black_box(
                poly_of_20vars
                    .clone()
                    .partial_evaluate(a_target, Fq::from(2)),
            )
        });
    });

    // evaluate
    group.bench_function("evaluate_for_2vars", |b| {
        b.iter(|| black_box(poly.clone().evaluate(&vec![Fq::from(2), Fq::from(3)])))
    });
    group.bench_function("evaluate_for_3vars", |b| {
        b.iter(|| {
            black_box(
                poly_of_3vars
                    .clone()
                    .evaluate(&vec![Fq::from(4), Fq::from(2), Fq::from(3)]),
            )
        })
    });
    group.bench_function("evaluate_for_10vars", |b| {
        b.iter(|| black_box(poly_of_10vars.clone().evaluate(&gen_random_vars(10))))
    });
    group.bench_function("evaluate_for_20vars", |b| {
        b.iter(|| black_box(poly_of_20vars.clone().evaluate(&gen_random_vars(20))))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

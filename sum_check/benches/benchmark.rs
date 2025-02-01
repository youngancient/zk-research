use ark_bn254::Fq;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use multilinear::evaluation_form::{gen_based_on_two, gen_random_vars, EvaluationForm};
use sum_check::sum_check::{get_sum_at_0_and_1, prove, verify};

fn benchmark(c: &mut Criterion) -> () {
    let mut poly = EvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(5)]);
    let sum1 = get_sum_at_0_and_1(&poly.eval_form);
    let proof = prove(&mut poly.clone(), sum1);

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
    let sum_of_3 = get_sum_at_0_and_1(&poly_of_3vars.eval_form);
    let proof_of_3 = prove(&mut poly_of_3vars.clone(), sum_of_3);

    let mut poly_of_10vars = EvaluationForm::<Fq>::new(gen_based_on_two(10));
    let sum_of_10 = get_sum_at_0_and_1(&poly_of_10vars.eval_form);
    let proof_of_10 = prove(&mut poly_of_10vars.clone(), sum_of_10);

    let mut poly_of_20vars = EvaluationForm::<Fq>::new(gen_based_on_two(20));
    let sum_of_20 = get_sum_at_0_and_1(&poly_of_10vars.eval_form);
    let proof_of_20 = prove(&mut poly_of_20vars.clone(), sum_of_20);

    let mut group = c.benchmark_group("sum_check");

    // benching prove
    group.bench_function("prove poly of 2vars", |b| {
        b.iter(|| black_box(prove(&mut poly.clone(), sum1)));
    });
    group.bench_function("prove poly of 3vars", |b| {
        b.iter(|| black_box(prove(&mut poly_of_3vars.clone(), sum_of_3)));
    });
    group.bench_function("prove poly of 10vars", |b| {
        b.iter(|| black_box(prove(&mut poly_of_10vars.clone(), sum_of_10)));
    });
    group.bench_function("prove poly of 20vars", |b| {
        b.iter(|| black_box(prove(&mut poly_of_20vars.clone(), sum_of_20)));
    });

    // benching verify
    group.bench_function("verify poly of 2vars", |b| {
        b.iter(|| black_box(verify(proof.clone(), &mut poly)));
    });
    group.bench_function("verify poly of 3vars", |b| {
        b.iter(|| black_box(verify(proof_of_3.clone(), &mut poly_of_3vars)));
    });
    group.bench_function("verify poly of 10vars", |b| {
        b.iter(|| black_box(verify(proof_of_10.clone(), &mut poly_of_10vars)));
    });
    group.bench_function("verify poly of 20vars", |b| {
        b.iter(|| black_box(verify(proof_of_20.clone(), &mut poly_of_20vars)));
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

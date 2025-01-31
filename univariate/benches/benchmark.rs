use ark_bn254::Fq;
use ark_ff::PrimeField;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::thread_rng;
use univariate::dense_polynomial::UnivariatePolynomialDense;

fn benchmark_function(c: &mut Criterion) -> () {
    let mut group = c.benchmark_group("sum_functions");
    let poly = UnivariatePolynomialDense::new(vec![Fq::from(1), Fq::from(2), Fq::from(3)]);
    let poly2 = UnivariatePolynomialDense::new(vec![Fq::from(5), Fq::from(2)]);

    let poly3 = UnivariatePolynomialDense::new(gen_random_vars(20));
    let poly4 = UnivariatePolynomialDense::new(gen_random_vars(20));

    group.bench_function("polynomial_multiplication", |b| {
        b.iter(|| black_box(&poly).polynomial_multiplication(black_box(&poly2)))
    });
    group.bench_function("polynomial_multiplication_for_20vars", |b| {
        b.iter(|| black_box(&poly3).polynomial_multiplication(black_box(&poly4)))
    });
    group.bench_function("evaluate", |b| {
        b.iter(|| black_box(poly.evaluate(Fq::from(2))))
    });
    group.bench_function("evaluate_for_20vars", |b| {
        b.iter(|| black_box(poly3.evaluate(Fq::from(2))))
    });
    group.bench_function("scalar_multiplication", |b| {
        b.iter(|| black_box(poly.scalar_multiplication(Fq::from(2))))
    });
    group.bench_function("scalar_multiplication_for_20vars", |b| {
        b.iter(|| black_box(poly3.scalar_multiplication(Fq::from(2))))
    });
    group.bench_function("interpolate", |b| {
        b.iter(|| {
            black_box(UnivariatePolynomialDense::interpolate(
                vec![Fq::from(0), Fq::from(1), Fq::from(2)],
                vec![Fq::from(2), Fq::from(4), Fq::from(10)],
            ))
        });
    });
    group.finish();
}

pub fn gen_random_vars<F: PrimeField>(n: u32) -> Vec<F> {
    let mut rng = thread_rng();
    let mut vars_list: Vec<F> = Vec::new();
    for _ in 0..n {
        let y: F = F::rand(&mut rng);
        vars_list.push(y);
    }
    vars_list
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);

use ark_bn254::Fq;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use univariate::dense_polynomial::UnivariatePolynomialDense;

fn benchmark_function(c: &mut Criterion) -> () {
    let mut group = c.benchmark_group("sum_functions");
    let poly = UnivariatePolynomialDense::new(vec![Fq::from(1), Fq::from(2), Fq::from(3)]);
    let poly2 = UnivariatePolynomialDense::new(vec![Fq::from(5), Fq::from(2)]);
    group.bench_function("polynomial_multiplication", |b| {
        b.iter(|| black_box(&poly).polynomial_multiplication(black_box(&poly2)))
    });
    group.bench_function("evaluate", |b| {
        b.iter(|| black_box(poly.evaluate(Fq::from(2))))
    });
    group.bench_function("scalar_multiplication", |b| {
        b.iter(|| black_box(poly.scalar_multiplication(Fq::from(2))))
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

criterion_group!(benches, benchmark_function);
criterion_main!(benches);

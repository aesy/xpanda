use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use xpanda::Xpanda;

pub fn expand(c: &mut Criterion) {
    let content = include_str!("input.txt");
    let mut named_vars = HashMap::new();
    named_vars.insert(String::from("VAL"), String::from("named"));
    let xpanda = Xpanda::builder()
        .with_positional_vars(vec![String::from("one")])
        .with_named_vars(named_vars.clone())
        .build();

    c.bench_function("Xpanda::expand", |b| b.iter(|| xpanda.expand(content)));
}

criterion_group!(benches, expand);
criterion_main!(benches);

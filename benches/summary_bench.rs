use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait Summary {
    fn summarize(&self) -> String;
}

struct VmConfig {
    cpu: u32,
    memory: u32,
}

impl Summary for VmConfig {
    fn summarize(&self) -> String {
        format!("VM: {}C/{}MB", self.cpu, self.memory)
    }
}

// 静态分发函数
fn bench_static(item: &impl Summary) -> String {
    item.summarize()
}

// 动态分发函数
fn bench_dynamic(item: &dyn Summary) -> String {
    item.summarize()
}

fn compare_dispatch_performance(c: &mut Criterion) {
    let vm = VmConfig { cpu: 16, memory: 32768 };
    
    // 1. 创建一个 Benchmark 组，名字叫 "Dispatch_Methods"
    let mut group = c.benchmark_group("Dispatch_Methods");

    // 2. 在同一组内添加静态分发测试
    group.bench_function("Static_Dispatch", |b| {
        b.iter(|| bench_static(black_box(&vm)))
    });

    // 3. 在同一组内添加动态分发测试
    group.bench_function("Dynamic_Dispatch", |b| {
        b.iter(|| bench_dynamic(black_box(&vm as &dyn Summary)))
    });

    // 4. 结束组测试
    group.finish();
}

criterion_group!(benches, compare_dispatch_performance);
criterion_main!(benches);
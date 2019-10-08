#![feature(test)]
extern crate test;

mod tests {
    #[bench]
    fn bench_ep(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph08"));
    }

    #[bench]
    fn bench_ep_refactored(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system_refactored::run("graph_files/graph08"));
    }
}

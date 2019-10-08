#![feature(test)]
extern crate test;

mod tests {
    #[bench]
    fn most_basic(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph00"));
    }

    #[bench]
    fn chain_one(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph09"));
    }

    #[bench]
    fn hard_one(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph07"));
    }
}

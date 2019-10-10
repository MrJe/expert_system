#![feature(test)]
extern crate test;

mod tests {
    #[bench]
    fn most_basic(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph00", &lib::options::Options::new()));
    }

    #[bench]
    fn chain_one(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph09", &lib::options::Options::new()));
    }

    #[bench]
    fn hard_one(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph07", &lib::options::Options::new()));
    }

    #[bench]
    fn expert_one(b: &mut test::Bencher) {
        b.iter(|| lib::expert_system::run("graph_files/graph42", &lib::options::Options::new()));
    }
}

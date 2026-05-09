use std::collections::HashMap;
use std::time::{Duration, Instant};

use criterion::{Bencher, Criterion, criterion_group, criterion_main};
use stringdex::internals::fs::Filesystem;
use stringdex::internals::tree::{SearchTree, encode_search_tree_ukkonen};
use stringdex::internals::write_tree;

#[derive(Default)]
struct InMemFs {
    hm: HashMap<String, Vec<u8>>,
    read_duration: Duration,
    write_duration: Duration,
}

impl Filesystem for InMemFs {
    fn read_file(&mut self, file_name: &str) -> std::io::Result<Option<Vec<u8>>> {
        let start = Instant::now();
        let file = self.hm.get(file_name).cloned();
        self.read_duration += start.elapsed();
        Ok(file)
    }

    fn write_file(&mut self, file_name: &str, contents: &[u8]) -> std::io::Result<()> {
        let start = Instant::now();
        self.hm.insert(file_name.to_owned(), contents.to_owned());
        self.write_duration += start.elapsed();
        Ok(())
    }
}

fn bench_write_tree(b: &mut Bencher, tree: &SearchTree) {
    b.iter_custom(|iterations| {
        let mut main_duration = Duration::ZERO;
        let mut write_duration = Duration::ZERO;
        let mut read_duration = Duration::ZERO;

        for _ in 0..iterations {
            let mut fs = InMemFs::default();

            let iter_start = Instant::now();
            let res = write_tree(tree, &mut fs);
            main_duration += iter_start.elapsed();
            let _ = res.unwrap();

            write_duration += fs.write_duration;
            read_duration += fs.read_duration;
        }

        main_duration
            .checked_sub(write_duration)
            .unwrap()
            .checked_sub(read_duration)
            .unwrap()
    })
}

pub fn write_tree_words_mini(c: &mut Criterion) {
    let dataset = std::fs::read("words.mini")
        .or_else(|_| std::fs::read("benches/words.mini"))
        .unwrap();
    let dataset: Vec<&[u8]> = dataset.split(|c| *c == b'\n').collect();
    let tree = encode_search_tree_ukkonen(dataset.iter().map(|x| &x[..]));
    c.benchmark_group("mini")
        .sample_size(200)
        .measurement_time(Duration::from_secs(10))
        .bench_function("write_tree", |b| {
            bench_write_tree(b, &tree);
        });
}

pub fn write_tree_words(c: &mut Criterion) {
    let dataset = std::fs::read("words")
        .or_else(|_| std::fs::read("benches/words"))
        .unwrap();
    let dataset: Vec<&[u8]> = dataset.split(|c| *c == b'\n').collect();
    let tree = encode_search_tree_ukkonen(dataset.iter().map(|x| &x[..]));
    c.benchmark_group("full")
        .sample_size(15)
        .measurement_time(Duration::from_secs(30))
        .bench_function("write_tree", |b| {
            bench_write_tree(b, &tree);
        });
}

criterion_group!(benches, write_tree_words_mini, write_tree_words);
criterion_main!(benches);

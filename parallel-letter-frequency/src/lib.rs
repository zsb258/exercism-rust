use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let mut freq = HashMap::new();

        let mut handles: Vec<_> = Vec::with_capacity(worker_count);
        for chunk in input.chunks(input.len() / worker_count + 1) {
            let handle = s.spawn(move || frequency_single_thread(chunk));
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap().into_iter().for_each(|(k, v)| {
                freq.entry(k).and_modify(|old| *old += v).or_insert(v);
            });
        }

        freq
    })
}

fn frequency_single_thread(input: &[&str]) -> HashMap<char, usize> {
    let mut freq = HashMap::new();

    input
        .iter()
        .flat_map(|line| line.chars())
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(c.to_lowercase().next())
            } else {
                None
            }
        })
        .for_each(|op| match op {
            Some(c) => {
                (*freq.entry(c).or_default()) += 1;
            }
            _ => (),
        });

    freq
}

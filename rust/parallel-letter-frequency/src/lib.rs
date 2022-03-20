use rayon::prelude::*;
use std::collections::HashMap;

use std::cmp::min;
use std::sync::Arc;
use std::thread;

pub fn frequency_1(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in input {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}

pub fn frequency_2(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|str| str.chars())
        .filter(|char| char.is_alphabetic())
        .map(|char| char.to_ascii_lowercase())
        .fold(HashMap::<char, usize>::new(), |mut acc, x| {
            let counter = acc.entry(x).or_insert(0);
            *counter += 1;
            acc
        })
}

pub fn frequency_3(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .par_iter()
        .map(|x| {
            x.chars()
                .filter(|char| char.is_alphabetic())
                .map(|char| char.to_ascii_lowercase())
                .fold(HashMap::<char, usize>::new(), |mut acc, x| {
                    let counter = acc.entry(x).or_default();
                    *counter += 1;
                    acc
                })
        })
        .reduce(HashMap::<char, usize>::new, |mut a, b| {
            b.into_iter().for_each(|(char, b_counter)| {
                let a_counter = a.entry(char).or_default();
                *a_counter += b_counter;
            });
            a
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::<char, usize>::new();
    }
    let input = input.join("");
    if input.is_empty() {
        return HashMap::<char, usize>::new();
    }
    let mut churn = input.chars();
    let real_worker_count = min(input.len(), worker_count);
    let work_length = (input.len() / real_worker_count).max(1);
    let work_length = if work_length * real_worker_count < input.len() {
        work_length + 1
    } else {
        work_length
    };

    (0..real_worker_count)
        .map(|_x| churn.by_ref().take(work_length).collect::<String>())
        .par_bridge()
        .map(|chunk| {
            chunk
                .chars()
                .filter(|char| char.is_alphabetic())
                .map(|char| char.to_ascii_lowercase())
                .fold(HashMap::<char, usize>::new(), |mut acc, x| {
                    let counter = acc.entry(x).or_default();
                    *counter += 1;
                    acc
                })
        })
        .reduce(HashMap::<char, usize>::new, |mut a, b| {
            b.into_iter().for_each(|(char, b_counter)| {
                let a_counter = a.entry(char).or_default();
                *a_counter += b_counter;
            });
            a
        })
}

pub fn frequency_4(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .par_iter()
        .flat_map(|str| str.chars().par_bridge())
        .filter(|char| char.is_alphabetic())
        .map(|char| char.to_ascii_lowercase())
        .fold(HashMap::<char, usize>::new, |mut acc, x| {
            let counter = acc.entry(x).or_insert(0);
            *counter += 1;
            acc
        })
        .reduce(HashMap::<char, usize>::new, |mut a, b| {
            b.into_iter().for_each(|(char, b_counter)| {
                let a_counter = a.entry(char).or_insert(0);
                *a_counter += b_counter;
            });
            a
        })
}

pub fn frequency_5(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answers = HashMap::<char, usize>::new();
    if input.is_empty() {
        return answers;
    }
    let input = input.join("");
    if input.is_empty() {
        return answers;
    }
    let mut churn = input.chars();
    let real_worker_count = min(input.len(), worker_count);
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let work_length = (input.len() / real_worker_count).max(1);
    let work_length = if work_length * real_worker_count < input.len() {
        work_length + 1
    } else {
        work_length
    };
    for _ in 0..real_worker_count {
        let chunk = churn.by_ref().take(work_length).collect::<String>();
        let my_thread = thread::spawn(move || {
            let mut answer = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    *answer.entry(c.to_ascii_lowercase()).or_default() += 1;
                }
            });
            answer
        });
        thread_pool.push(my_thread);
    }
    for my_thread in thread_pool {
        let answer = my_thread.join().unwrap();
        for (key, val) in answer.iter() {
            *answers.entry(*key).or_default() += val;
        }
    }
    answers
}

pub fn frequency_6(text: &[&str], n: usize) -> HashMap<char, usize> {
    let data: Vec<String> = text.iter().map(|s| s.to_lowercase()).collect();
    let arc = Arc::new(data);
    let mut threads = vec![];
    for i in 0..n {
        let data = arc.clone();
        threads.push(thread::spawn(move || {
            let mut counts = HashMap::new();
            let mut k = i;
            while k < data.len() {
                for c in data.get(k).unwrap().chars().filter(|c| c.is_alphabetic()) {
                    *counts.entry(c).or_insert(0) += 1;
                }
                k += n;
            }
            counts
        }));
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    for thread in threads {
        for (k, v) in thread.join().unwrap() {
            *counts.entry(k).or_insert(0) += v;
        }
    }
    counts
}

pub fn frequency_7(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    (0..worker_count)
        .into_par_iter()
        .map(|i| {
            let mut tallies = HashMap::new();
            input
                .join("")
                .chars()
                .skip(i)
                .step_by(worker_count)
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
                .for_each(|c| {
                    *tallies.entry(c).or_insert(0) += 1;
                });
            tallies
        })
        .reduce(HashMap::new, |mut result, m| {
            m.iter().for_each(|(&k, &v)| {
                *result.entry(k).or_insert(0) += v;
            });
            result
        })
}

pub fn frequency_8(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|str| str.chars())
        .collect::<Vec<char>>()
        .par_iter()
        .filter(|char| char.is_alphabetic())
        .map(|char| char.to_ascii_lowercase())
        .fold(HashMap::<char, usize>::new, |mut acc, x| {
            let counter = acc.entry(x).or_insert(0);
            *counter += 1;
            acc
        })
        .reduce(HashMap::<char, usize>::new, |mut a, b| {
            b.into_iter().for_each(|(char, b_counter)| {
                let a_counter = a.entry(char).or_insert(0);
                *a_counter += b_counter;
            });
            a
        })
}

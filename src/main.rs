use std::fs::File;
use std::io::Write;
use std::num;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

// Function for finding the optimal ranges for splitting up work
// Explanation: For every number 'x' checked, sqrt(x) operations occur meaning
// the total work done is equal to the area under the function:
//   f(n) = sqrt(n)
// Solve for n segments using Anti-derivative which yields:
// a_i = (n_i^(2/3) * limit) / 4 (for 8 segments)
fn find_equal_sections(num_segments: u32, total: u32) -> Vec<u32> {
    let mut ratios: Vec<u32> = vec![2];
    for i in 1..(num_segments + 1) {
        // solve
        let term = (i as f32).powf(2.0).powf(1.0 / 3.0); // n_i^(2/3)
        let a = (term / 4.0) * total as f32;
        ratios.push(a.floor() as u32);
    }
    return ratios;
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f32).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let max = 100_000_000;
    let num_threads = 8;
    let mut children = vec![];

    let start = Instant::now();
    let ranges: Vec<u32> = find_equal_sections(num_threads, max);

    // multi-threaded execution
    for i in 0..num_threads {
        let tx = tx.clone();
        let thread_start: u32 = ranges[i as usize];
        let thread_end: u32 = ranges[(i + 1) as usize];

        let child = thread::spawn(move || {
            let mut primes = Vec::new();
            if i == 0 {primes.push(2);}
            for n in thread_start..=thread_end {
                if n % 2 != 0 && is_prime(n as u32) {
                    primes.push(n);
                }
            }
            let th_finished_time = start.elapsed();
            println!("Thread #{i} finished check in {:?}!", th_finished_time);
            tx.send(primes).unwrap();
        });
        children.push(child);
    }

    let mut all_primes = Vec::new();
    for _ in 0..num_threads {
        let primes = rx.recv().unwrap();
        all_primes.extend(primes);
    }

    for child in children {
        child.join().unwrap();
    }

    all_primes.sort_unstable();
    let mut total_count = all_primes.len();
    let mut total_sum = all_primes.iter().fold(0u64, |sum, i| sum + (*i as u64));
    let largest_primes = &all_primes[total_count - 10..];

    let duration = start.elapsed();

    let mut file = File::create("primes.txt").unwrap();
    write!(file, "{:?} ", duration).unwrap();
    write!(file, "{} ", total_count).unwrap();
    write!(file, "{} ", total_sum).unwrap();
    writeln!(file, "{:?}", largest_primes).unwrap();
}

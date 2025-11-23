use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = (1..=1_000).map(|n| n as i64).collect::<Vec<i64>>(); // Sample data: numbers from 1 to 1000

    let shared_data = Arc::new(data); // Shared ownership of the data vector across threads

    let total = Arc::new(Mutex::new(0i64)); // Mutex to protect the total sum across threads

    let num_threads = 8; // Number of threads to spawn for computation
    let mut handles = Vec::with_capacity(num_threads); // Vector to hold thread handles

    let chunk_size = (shared_data.len() + num_threads - 1) / num_threads; // Calculate chunk size for each thread

    for thread_idx in 0..num_threads { // Spawn threads to compute partial sums
        let data_clone = Arc::clone(&shared_data); // Clone the Arc for data for the thread
        let handle = thread::spawn(move || { // Thread computation closure
            let start = thread_idx * chunk_size; // Calculate start index for this thread
            let end = ((thread_idx + 1) * chunk_size).min(data_clone.len()); // Calculate end index for this thread

            if start >= end {
                return 0; // No work for this thread if the range is invalid
            }

            let mut partial = 0i64; // Variable to hold the partial sum for this thread
            for &value in &data_clone[start..end] { // Iterate over the assigned chunk of data
                partial += value; // Accumulate the partial sum
            }

            partial // Return the partial sum
        });

        handles.push(handle); // Store the thread handle for later joining
    }

    let mut total = 0i64; // Variable to hold the final total sum
    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("Final total (with Mutex) = {}", total);
}

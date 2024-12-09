use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

// Function to read a CSV file and convert it into a 2D vector using threads
fn read_csv_to_2d_vector(file_path: &str) -> Vec<Vec<String>> {
    // Open the file
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    // Shared vector to store the CSV data
    let csv_data = Arc::new(Mutex::new(Vec::new()));

    // Vector to hold the thread handles
    let mut handles = vec![];

    // Read the file line by line
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let csv_data = Arc::clone(&csv_data);

        // Spawn a new thread for each line
        let handle = thread::spawn(move || {
            let mut row = Vec::new();
            for value in line.split(',') {
                row.push(value.to_string());
            }
            let mut csv_data = csv_data.lock().unwrap();
            csv_data.push(row);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Return the CSV data
    Arc::try_unwrap(csv_data).expect("Arc has multiple owners").into_inner().expect("Mutex cannot be unlocked")
}

fn write_2d_vector_to_csv(file_path: &str, data: Vec<Vec<String>>) {
    // Create a file to write
    let file = File::create(file_path).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for row in data {
        let line = row.join(",");
        writeln!(writer, "{}", line).expect("Unable to write line");
    }
}



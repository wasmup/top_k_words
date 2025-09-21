//  Write a function that takes a list of log lines (strings) and
// return the k most frequently occurring words, sorted by frequency (descending).
// Ignore case (Error and error should be treated the same).
// Words are sequences of alphanumeric characters (a-z, A-Z, 0-9).
// If multiple words have the same frequency, sort them alphabetically.
// The function signature should look like:
// fn top_k_words(logs: &[String], k: usize) -> Vec<(String, usize)>;

// Example Input:
// let logs = vec![
//  "Error: Disk full".to_string(),
//  "Warning: Memory low".to_string(),
//  "error: network down".to_string(),
//  "Error: Disk full".to_string(),
// ];
// let result = top_k_words(&logs, 2);

// Expected Output:
// [("error", 3), ("disk", 2)]
//
// O(n log n)
fn top_k_words(lines: &[String], k: usize) -> Vec<(String, usize)> {
    if k == 0 || lines.is_empty() {
        return Vec::new();
    }

    let mut word = String::new();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for line in lines {
        for &b in line.as_bytes() {
            if b'A' <= b && b <= b'Z' {
                word.push((b | 32) as char); // lowercase
            } else if (b'a' <= b && b <= b'z') || (b'0' <= b && b <= b'9') {
                word.push(b as char);
            } else {
                if !word.is_empty() {
                    let w = std::mem::take(&mut word);
                    *counts.entry(w).or_insert(0) += 1;
                }
            }
        }
        if !word.is_empty() {
            let w = std::mem::take(&mut word);
            *counts.entry(w).or_insert(0) += 1;
        }
    }

    let mut items: Vec<(String, usize)> = counts.into_iter().collect();
    items.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    items.into_iter().take(k).collect()
}

fn main() {
    let logs = vec![
        "Error: Disk full".to_string(),
        "Warning: Memory low".to_string(),
        "error: network down".to_string(),
        "Error: Disk full".to_string(),
    ];

    println!("{:?}", top_k_words(&logs, 2));
    // [("error", 3), ("disk", 2)]
}

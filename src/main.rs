use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::time::Instant;

fn analyze_text_slow(text: &str) -> TextStats {
    let start = Instant::now();

    let mut word_freq = HashMap::new();
    for line in text.lines() {
        for word in line.split_whitespace() {
            let clean_word = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();

            if !clean_word.is_empty() {
                *word_freq.entry(clean_word.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut word_vec: Vec<_> = word_freq.iter().collect();
    word_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let top_words: Vec<(String, usize)> = word_vec
        .into_iter()
        .take(10)
        .map(|(w, c)| (w.clone(), *c))
        .collect();

    let mut char_count = 0;
    for line in text.lines() {
        for ch in line.chars() {
            if ch.is_alphabetic() {
                char_count += 1;
            }
        }
    }

    let mut all_words = Vec::new();
    for line in text.lines() {
        for word in line.split_whitespace() {
            let clean = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            if !clean.is_empty() {
                all_words.push(clean);
            }
        }
    }

    all_words.sort_by(|a, b| b.len().cmp(&a.len()));
    let longest_words: Vec<String> = all_words.iter()
        .take(5)
        .map(|s| s.clone())
        .collect();

    TextStats {
        word_count: word_freq.len(),
        char_count,
        top_words,
        longest_words,
        time_ms: start.elapsed().as_millis(),
    }
}

fn analyze_text_fast(text: &str) -> TextStats {
    let start = Instant::now();

    let mut word_freq: HashMap<String, usize> = HashMap::with_capacity(10000);
    let mut char_count = 0;
    let mut longest_words_heap: BinaryHeap<Reverse<(usize, String)>> = BinaryHeap::new();

    for word in text.split_ascii_whitespace() {
        let mut clean_word = String::with_capacity(word.len());
        for &ch in word.as_bytes() {
            if ch.is_ascii_alphabetic() {
                char_count += 1;
                clean_word.push((ch | 0x20) as char);
            }
        }

        if !clean_word.is_empty() {
            let len = clean_word.len();
            
            if longest_words_heap.len() < 5 {
                longest_words_heap.push(Reverse((len, clean_word.clone())));
            } else if let Some(&Reverse((min_len, _))) = longest_words_heap.peek() {
                if len > min_len {
                    longest_words_heap.pop();
                    longest_words_heap.push(Reverse((len, clean_word.clone())));
                }
            }
            
            *word_freq.entry(clean_word).or_insert(0) += 1;
        }
    }

    let word_count = word_freq.len();
    
    let mut top_words_heap: BinaryHeap<(usize, String)> = word_freq
        .into_iter()
        .map(|(w, c)| (c, w))
        .collect();
    
    let mut top_words = Vec::with_capacity(10);
    for _ in 0..10 {
        if let Some((count, word)) = top_words_heap.pop() {
            top_words.push((word, count));
        }
    }
    
    let mut longest_vec: Vec<_> = longest_words_heap.into_iter().map(|Reverse(x)| x).collect();
    longest_vec.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let longest_words: Vec<String> = longest_vec.into_iter().map(|(_, w)| w).collect();

    TextStats {
        word_count,
        char_count,
        top_words,
        longest_words,
        time_ms: start.elapsed().as_millis(),
    }
}

#[derive(Debug)]
struct TextStats {
    word_count: usize,
    char_count: usize,
    top_words: Vec<(String, usize)>,
    longest_words: Vec<String>,
    time_ms: u128,
}

fn generate_test_text(size: usize) -> String {
    let base_words = vec![
        "rust", "performance", "optimization", "memory", "speed", "efficiency",
        "benchmark", "algorithm", "data", "structure", "programming", "language",
        "system", "compile", "zero", "cost", "abstraction", "ownership", "borrow",
        "lifetime", "trait", "generic", "macro", "unsafe", "async", "await",
        "concurrency", "parallelism", "thread", "mutex", "channel", "vector",
        "hashmap", "iterator", "closure", "pattern", "matching", "error", "handling",
        "result", "option", "reference", "pointer", "stack", "heap", "allocation",
        "deallocation", "garbage", "collection", "cargo", "crate", "module",
        "function", "method", "struct", "enum", "impl", "type", "inference",
        "syntax", "semantic", "compiler", "llvm", "inline",
        "monomorphization", "specialization", "documentation", "test", "integration"
    ];
    
    (0..size)
        .map(|i| {
            if i % 20 < 19 {
                format!("uniqueword{}", i)
            } else {
                base_words[i % base_words.len()].to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let text = generate_test_text(50_000);

    println!("Analyzing {} bytes of text...\n", text.len());

    let stats_slow = analyze_text_slow(&text);
    println!("SLOW VERSION:");
    println!("  Unique words: {}", stats_slow.word_count);
    println!("  Total chars: {}", stats_slow.char_count);
    println!("  Top 10 words: {:?}", stats_slow.top_words);
    println!("  Longest words: {:?}", stats_slow.longest_words);
    println!("  Time: {} ms\n", stats_slow.time_ms);

    let stats_fast = analyze_text_fast(&text);
    println!("FAST VERSION:");
    println!("  Unique words: {}", stats_fast.word_count);
    println!("  Total chars: {}", stats_fast.char_count);
    println!("  Top 10 words: {:?}", stats_fast.top_words);
    println!("  Longest words: {:?}", stats_fast.longest_words);
    println!("  Time: {} ms", stats_fast.time_ms);
    
    let speedup = stats_slow.time_ms as f64 / stats_fast.time_ms.max(1) as f64;
    println!("\nSpeedup: {:.1}x faster!", speedup);
}

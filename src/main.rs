use std::time::Instant;

const MAX_WORD_LEN: usize = 32;
const MAX_UNIQUE_WORDS: usize = 128;

struct WordTable {
    words: Vec<u8>,
    offsets: Vec<(u32, u16)>,
    counts: Vec<u32>,
}

impl WordTable {
    fn new() -> Self {
        Self {
            words: Vec::with_capacity(2048),
            offsets: Vec::with_capacity(MAX_UNIQUE_WORDS),
            counts: Vec::with_capacity(MAX_UNIQUE_WORDS),
        }
    }

    #[inline(always)]
    fn find_or_insert(&mut self, word: &[u8]) -> usize {
        for (i, &(offset, len)) in self.offsets.iter().enumerate() {
            let start = offset as usize;
            let end = start + len as usize;
            if &self.words[start..end] == word {
                return i;
            }
        }
        
        let offset = self.words.len() as u32;
        let len = word.len() as u16;
        self.words.extend_from_slice(word);
        self.offsets.push((offset, len));
        self.counts.push(0);
        self.offsets.len() - 1
    }

    fn get_word(&self, idx: usize) -> &str {
        let (offset, len) = self.offsets[idx];
        let start = offset as usize;
        let end = start + len as usize;
        unsafe { std::str::from_utf8_unchecked(&self.words[start..end]) }
    }
}

#[inline(always)]
fn process_word_bytes(word: &[u8], buffer: &mut Vec<u8>) -> usize {
    buffer.clear();
    let mut char_count = 0;
    
    for &byte in word {
        if byte.is_ascii_alphabetic() {
            buffer.push(byte.to_ascii_lowercase());
            char_count += 1;
        }
    }
    
    char_count
}

fn analyze_text_slow(text: &str) -> TextStats {
    let mut word_table = WordTable::new();
    let mut longest_5 = [(0usize, 0u16); 5];
    let mut longest_count = 0;
    let mut char_count = 0;

    let mut buffer = Vec::with_capacity(MAX_WORD_LEN);
    let bytes = text.as_bytes();
    let mut i = 0;
    let len = bytes.len();
    
    while i < len {
        while i < len && bytes[i] <= b' ' {
            i += 1;
        }
        
        if i >= len { break; }
        
        let word_start = i;
        while i < len && bytes[i] > b' ' {
            i += 1;
        }
        
        char_count += process_word_bytes(&bytes[word_start..i], &mut buffer);

        if !buffer.is_empty() {
            let word_idx = word_table.find_or_insert(&buffer);
            word_table.counts[word_idx] += 1;
            
            let word_len = buffer.len();
            
            if longest_count < 5 {
                longest_5[longest_count] = (word_len, word_idx as u16);
                longest_count += 1;
            } else {
                let min_len = longest_5[0].0.min(longest_5[1].0).min(longest_5[2].0).min(longest_5[3].0).min(longest_5[4].0);
                if word_len > min_len {
                    let min_idx = (0..5).min_by_key(|&i| longest_5[i].0).unwrap();
                    longest_5[min_idx] = (word_len, word_idx as u16);
                }
            }
        }
    }

    let word_count = word_table.offsets.len();
    let mut top_words: Vec<(String, usize)> = Vec::with_capacity(word_count);
    
    for i in 0..word_count {
        top_words.push((word_table.get_word(i).to_string(), word_table.counts[i] as usize));
    }
    
    top_words.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    top_words.truncate(10);

    longest_5[..longest_count].sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let longest_words: Vec<String> = longest_5[..longest_count]
        .iter()
        .map(|(_, idx)| word_table.get_word(*idx as usize).to_string())
        .collect();

    TextStats {
        word_count,
        char_count,
        top_words,
        longest_words,
    }
}

#[derive(Debug)]
struct TextStats {
    word_count: usize,
    char_count: usize,
    top_words: Vec<(String, usize)>,
    longest_words: Vec<String>,
}

fn generate_test_text(size: usize) -> String {
    let words = vec!["rust", "performance", "optimization", "memory", "speed",
                     "efficiency", "benchmark", "algorithm", "data", "structure"];

    (0..size)
        .map(|i| words[i % words.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let text = generate_test_text(50_000);

    println!("Analyzing {} bytes of text...\n", text.len());
    
    let start = Instant::now();
    let stats = analyze_text_slow(&text);
    let elapsed = start.elapsed().as_micros();

    println!("Results:");
    println!("  Unique words: {}", stats.word_count);
    println!("  Total chars: {}", stats.char_count);
    println!("  Top 10 words: {:?}", stats.top_words);
    println!("  Longest words: {:?}", stats.longest_words);
    println!("\n⏱️  Time taken: {} µs ({:.2} ms)", elapsed, elapsed as f64 / 1000.0);
}


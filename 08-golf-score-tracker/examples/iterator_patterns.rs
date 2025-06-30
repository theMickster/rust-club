//! Iterator Patterns Example
//! 
//! This example teaches Rust iterator patterns through golf scenarios:
//! - map() - Transform data
//! - filter() - Conditional selection
//! - filter_map() - Transform + filter
//! - fold() - Accumulation
//! - collect() - Build collections
//!
//! Run with: cargo run --example iterator_patterns

fn main() {
    println!("🦀 Rust Iterator Patterns - Golf Edition\n");
    let scores: Vec<(u8, u8)> = vec![
        (1, 3), (2, 6), (3, 3), (4, 4), (5, 6),
        (6, 3), (7, 4), (8, 5), (9, 3),
    ];
    let pars: Vec<u8> = vec![4, 5, 3, 4, 5, 3, 4, 3, 5];

    println!("══════════════════════════════════════════════════════════════════════════════");
    println!("📌  Pattern 1: map() - Transform Each Item");
    println!("══════════════════════════════════════════════════════════════════════════════");
    let strokes: Vec<u8> = scores.iter().map( |(_hole, strokes)| *strokes ).collect();
    println!("Scores: {:?}", strokes);

    println!("\n══════════════════════════════════════════════════════════════════════════════");
    println!("📌  Pattern 2: filter() - Keep Matching Items");
    println!("══════════════════════════════════════════════════════════════════════════════");
    let great_scores: Vec<_> = scores.iter()
        .filter(|&(_hole, strokes)| *strokes < 4)
        .collect();
    println!("🥇 Great Scores (under 4 strokes): {:?}", great_scores);

    println!("\n══════════════════════════════════════════════════════════════════════════════");
    println!("📌  Pattern 3: filter_map() - Transform + Filter");
    println!("══════════════════════════════════════════════════════════════════════════════");
    let under_par: Vec<_> = scores
        .iter()
        .enumerate()
        .filter_map(| (idx, (_hole, strokes) )| {
            let par = pars[idx];
            let diff = *strokes as i8 - par as i8;
            if diff < 0 {
                Some((idx + 1, diff))
            } else {
                None
            }
        })
        .collect();
    println!("📣 Under par holes: {:?}", under_par);

    println!("\n══════════════════════════════════════════════════════════════════════════════");
    println!("📌  Pattern 4: fold() - Accumulate");
    println!("══════════════════════════════════════════════════════════════════════════════");
    let (count_eagles , count_birdies, count_pars, count_bogeys, count_double_bogeys) = scores
        .iter()
        .enumerate()
        .fold(
        (0, 0, 0, 0, 0),
        |(eagles, birdies, count_pars, bogeys, double_bogeys), (idx, (_hole, strokes))| {
            let par = pars[idx];
            match *strokes as i8 - par as i8 {
                -2 => (eagles + 1, birdies, count_pars, bogeys, double_bogeys),
                -1 => (eagles, birdies + 1, count_pars, bogeys, double_bogeys),
                0 => (eagles, birdies, count_pars + 1, bogeys, double_bogeys),
                1 => (eagles, birdies, count_pars, bogeys + 1, double_bogeys),
                2 => (eagles, birdies, count_pars, bogeys, double_bogeys + 1),
                _ => (eagles, birdies, count_pars, bogeys, double_bogeys),
            }
        });
    println!( "🏆 Performance ");
    println!("  🦅 Eagles: {}", count_eagles);
    println!("  🐦 Birdies: {}", count_birdies);
    println!("  ⛳ Pars: {}", count_pars);
    println!("  😒 Bogeys: {}", count_bogeys);
    println!("  💩 Double Bogeys: {}", count_double_bogeys);

    println!("\n══════════════════════════════════════════════════════════════════════════════");
    println!("📌  Pattern 5: Chaining Operations");
    println!("══════════════════════════════════════════════════════════════════════════════");
    let eagles: Vec<_> = scores
        .iter()
        .enumerate()
        .filter_map(|(idx, (hole, strokes))| {
            let par = pars[idx];
            let diff = *strokes as i8 - par as i8;
            if diff <= -2 {
                Some(*hole)
            } else {
                None
            }
        })
        .collect();
    
    println!("🦅 Eagle holes: {:?}", eagles);

    println!("\n══════════════════════════════════════════════════════════════════════════════");
    println!("📌  Pattern 6: min/max/sum");
    println!("══════════════════════════════════════════════════════════════════════════════");
    let total: u8 = strokes.iter().sum();
    let best: u8 = *strokes.iter().min().unwrap();
    let worst: u8 = *strokes.iter().max().unwrap();

    println!("Total: {}, Best: {}, Worst: {}", total, best, worst);
}
use phf::{phf_map, Map};

struct BoundedHash {
    map: &'static Map<&'static str, u8>,
    max_len: usize,
}

static PART1_MAP: Map<&'static str, u8> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
};

// Largest key in PART1_MAP
static PART1_KEY_WIDTH: usize = 1;

static PART1_HASH: BoundedHash = BoundedHash {
    map: &PART1_MAP,
    max_len: PART1_KEY_WIDTH,
};

static PART2_MAP: Map<&'static str, u8> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

// Largest key in PART2_MAP
static PART2_KEY_WIDTH: usize = 5;

static PART2_HASH: BoundedHash = BoundedHash {
    map: &PART2_MAP,
    max_len: PART2_KEY_WIDTH,
};

// Iterate through the input string &str, and output
// a list of digits that occur within it in-order
// according to the `BoundedHash`
//
//
fn str_to_digit(input: &str, bounded_hash: &'static BoundedHash) -> u64 {
    let len = input.len();
    let mut output = Vec::with_capacity(len);
    for start_idx in 0..len {
        for window_len in 1..=bounded_hash.max_len {
            let end_idx = start_idx + window_len;
            if let Some(slice) = &input.get(start_idx..end_idx) {
                if let Some(val) = bounded_hash.map.get(slice) {
                    output.push(*val);
                    break;
                }
            } else {
                break;
            }
        }
    }
    let first: u64 = output[0].into();
    let last: u64 = output[output.len() - 1].into();
    10 * first + last
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").expect("Input file not found");
    // Part 1
    let sum1: u64 = input
        .lines()
        .map(|line: &str| str_to_digit(line, &PART1_HASH))
        .sum();
    println!("Part 1: {}", sum1);
    // Part 2
    let sum2: u64 = input
        .lines()
        .map(|line: &str| str_to_digit(line, &PART2_HASH))
        .sum();
    println!("Part 2: {}", sum2);
}

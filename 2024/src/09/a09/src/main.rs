use std::io::Read;

fn get_checksum(digits: &[usize]) -> usize {
    let mut checksum = 0;

    // Represents where we're currently putting blocks
    let mut current_block = 0;

    // The digits representing the files that don't need to move
    let mut left_idx = 0;
    let mut left_digit = digits[left_idx];

    // The digits representing the files that do need to move
    let mut right_idx = digits.len() - 1;
    if right_idx % 2 == 1 {
        // Ignore empty space at the end
        right_idx -= 1;
    }
    let mut right_digit = digits[right_idx];

    while left_idx < right_idx {
        // Make sure that we're pointed to valid blocks
        if left_digit == 0 {
            left_idx += 1;
            left_digit = digits[left_idx];
            continue;
        } else if right_digit == 0 {
            right_idx -= 2;
            right_digit = digits[right_idx];
            continue;
        }

        if left_idx % 2 == 0 {
            // Count a block from the left side
            checksum += left_idx / 2 * current_block;

            left_digit -= 1;
            current_block += 1;
        } else {
            // Move a block from the right side
            checksum += right_idx / 2 * current_block;
            right_digit -= 1;
            left_digit -= 1;
            current_block += 1;
        }
    }
    if left_idx == right_idx {
        // Move any remaining blocks from the right side
        while right_digit > 0 {
            checksum += left_idx / 2 * current_block;
            right_digit -= 1;
            current_block += 1;
        }
    }
    checksum
}

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let digits: Vec<usize> = line
        .as_bytes()
        .into_iter()
        .map(|c| (c - b'0') as usize)
        .collect();

    let checksum = get_checksum(&digits);
    println!("{checksum}");
}

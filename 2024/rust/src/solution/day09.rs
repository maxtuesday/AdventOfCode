use super::Solution;

pub struct Day09;

#[derive(Clone, Copy, Debug)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug)]
enum Block {
    File { file: File },
    Free { size: usize },
}

fn checksum(layout: &[Option<usize>]) -> usize {
    layout
        .iter()
        .enumerate()
        .map(|(idx, maybe_id)| match maybe_id {
            Some(id) => idx * id,
            None => 0,
        })
        .sum()
}

fn move_file(layout: &mut Vec<Block>, r: usize) -> Option<()> {
    let target = match layout[r] {
        Block::File { file } => file,
        Block::Free { .. } => {
            unimplemented!("we shouldn't be able to end up with a an Block::Empty for 'r'")
        }
    };

    // iterate left until we find an Block::Empty with enough space to fit the target file.
    let (idx, block) = layout.iter().enumerate().find(|(_, block)| match block {
        Block::File { .. } => false,
        Block::Free { size } => *size >= target.size,
    })?;

    // check if next_free_block index is after the file.
    // If this is the case, then we should should not move the file
    if idx > r {
        return None;
    }

    let block_size = match block {
        Block::File { .. } => None,
        Block::Free { size } => Some(size),
    }?;

    // We can fit the full file inside this free block span.
    // Update the free block with the remaining space.
    let remaining_size = block_size - target.size;
    if remaining_size == 0 {
        // If the remaining space is 0, then we will remove the block entirely
        // Therefore, we can swap the file and free space blocks
        layout.swap(idx, r);
    } else {
        // Update free block with remaining space
        layout[idx] = Block::Free {
            size: remaining_size,
        };
        // Replace file index with free space block
        layout[r] = Block::Free { size: target.size };
        // Insert before with target free space block
        let file = Block::File {
            file: File {
                id: target.id,
                size: target.size,
            },
        };
        layout.insert(idx, file);
    }
    Some(())
}

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let mut id = 0usize;
        let mut layout = input
            .chars()
            .enumerate()
            .flat_map(|(i, char)| {
                let is_file = i % 2 == 0;
                let size = char.to_digit(10).expect("should be valid digit") as usize;
                if is_file {
                    id += 1; // subtrack 1 since we increment before use
                    vec![Some(id - 1); size]
                } else {
                    vec![None; size]
                }
            })
            .collect::<Vec<_>>();

        // fill in empty space from the front by taking filled space from the back
        let mut l = 0;
        let mut r = layout.len() - 1;
        while l < r {
            // set l to next free space from the left
            while layout[l].is_some() {
                l += 1;
            }
            // set r to the next free space from the right
            while layout[r].is_none() {
                r -= 1;
            }

            // double check if the left/right pointers have passed each other
            if l >= r {
                break;
            }

            // swap l and r
            layout.swap(l, r);
        }

        // checksum
        let checksum = checksum(&layout);
        format!("{checksum}")
    }

    fn part2(&self, input: &str) -> String {
        let mut id = 0usize;
        let mut layout = input
            .chars()
            .enumerate()
            .map(|(i, char)| {
                let is_file = i % 2 == 0;
                let size = char.to_digit(10).expect("should be valid digit") as usize;
                if is_file {
                    id += 1; // subtrack 1 since we increment before use
                    Block::File {
                        file: File { id: id - 1, size },
                    }
                } else {
                    Block::Free { size }
                }
            })
            .collect::<Vec<_>>();

        // Attempt to move whole files to the leftmost span of free space blocks that could fit the file.
        // Attempt to move each file exactly once in order of decreasing file ID number
        // starting with the file with the highest file ID number.
        // If there is no span of free space to the left of a file that
        // is large enough to fit the file, the file does not move.

        let mut current_file_id = id - 1; // This is the largest file ID we have.
        let mut r = layout.len() - 1;
        while 0 < r {
            // Get next file
            match layout[r] {
                Block::File { file } => {
                    if file.id == current_file_id {
                        move_file(&mut layout, r);
                        current_file_id -= 1;
                    }
                }
                Block::Free { .. } => {
                    // do nothing
                }
            }
            r -= 1; // move to next canidate file
        }

        // checksum
        let mut idx = 0;
        let checksum = layout
            .into_iter()
            .map(|block| match block {
                Block::File { file } => (0..file.size).fold(0, |acc, _| {
                    idx += 1;
                    acc + (idx - 1) * file.id
                }),
                Block::Free { size } => {
                    idx += size;
                    0
                }
            })
            .sum::<usize>();
        format!("{checksum}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_sample() {
        let expected = "1928";
        let actual = Day09.part1(INPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_sample() {
        let expected = "2858";
        let actual = Day09.part2(INPUT);
        assert_eq!(actual, expected);
    }
}

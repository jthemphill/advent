use std::io::Read;

struct File {
    starting_block: usize,
    length: usize,
    prev_id: Option<usize>,
    next_id: Option<usize>,
}

fn unlink(files: &mut Vec<File>, file_id: usize) {
    if let Some(prev_id) = files[file_id].prev_id {
        files[prev_id].next_id = files[file_id].next_id;
    }
    if let Some(next_id) = files[file_id].next_id {
        files[next_id].prev_id = files[file_id].prev_id;
    }
    files[file_id].prev_id = None;
    files[file_id].next_id = None;
}

fn link_after(files: &mut Vec<File>, file_id: usize, after_id: usize) {
    // Must unlink first
    assert!(files[file_id].prev_id.is_none());
    assert!(files[file_id].next_id.is_none());

    files[file_id].prev_id = Some(after_id);
    files[file_id].next_id = files[after_id].next_id;
    files[file_id].starting_block = files[after_id].starting_block + files[after_id].length;

    if let Some(next_id) = files[after_id].next_id {
        files[next_id].prev_id = Some(file_id);
    }
    files[after_id].next_id = Some(file_id);
}

fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let lengths: Vec<usize> = line
        .as_bytes()
        .into_iter()
        .map(|c| (c - b'0') as usize)
        .collect();

    let mut files = vec![];
    let mut current_block = 0;
    for (i, &length) in lengths.iter().enumerate() {
        if i % 2 == 0 {
            let id = i / 2;
            files.push(File {
                starting_block: current_block,
                length,
                prev_id: if id > 0 { Some(id - 1) } else { None },
                next_id: if i + 2 < lengths.len() {
                    Some(id + 1)
                } else {
                    None
                },
            });
            current_block += length;
        } else {
            current_block += length;
        }
    }

    let mut defrag_id = lengths.len() / 2;
    while defrag_id > 0 {
        let mut current_id = 0;
        while let Some(next_id) = files[current_id].next_id {
            assert!(files[current_id].starting_block < files[next_id].starting_block);

            let end_of_current_file = files[current_id].starting_block + files[current_id].length;
            let free_space = files[next_id].starting_block - end_of_current_file;
            if files[defrag_id].length <= free_space {
                // We can move the file
                println!(
                        "Moving {defrag_id} after {current_id} to block {end_of_current_file} ({} blocks)",
                        files[current_id].length,
                    );

                unlink(&mut files, defrag_id);
                link_after(&mut files, defrag_id, current_id);

                break;
            }
            current_id = next_id;
            if current_id == defrag_id {
                break;
            }
        }

        defrag_id -= 1;
    }

    let mut checksum = 0;
    let mut current_id = 0;
    loop {
        let starting_block = files[current_id].starting_block;
        let ending_block = starting_block + files[current_id].length;
        for block in starting_block..ending_block {
            checksum += current_id * block;
        }

        if let Some(next_id) = files[current_id].next_id {
            current_id = next_id;
        } else {
            break;
        }
    }
    println!("{checksum}");
}

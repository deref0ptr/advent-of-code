use std::{collections::BTreeMap, io};

fn main() -> io::Result<()> {
    let mut free_space = BTreeMap::new();

    let mut disk = day9::get_disk(io::stdin().lock(), |start, len| _ = free_space.insert(start, len))?;

    let mut src = disk.len() - 1;

    while src > 0 {
        match disk[src] {
            -1 => src -= 1,
            file_id => {
                let mut num_blocks = 1;

                while src > 0 && disk[src - 1] == file_id {
                    num_blocks += 1;
                    src -= 1;
                }

                if src == 0 {
                    break;
                };

                if let Some((&start, &len)) = free_space
                    .iter()
                    .find(|&(&start, &len)| len >= num_blocks && start + len <= src)
                {
                    let [a, b] = disk
                        .get_disjoint_mut([start..(start + num_blocks), src..(src + num_blocks)])
                        .unwrap();
                    a.swap_with_slice(b);

                    free_space.remove(&start);
                    let remaining = len - num_blocks;
                    if remaining > 0 {
                        free_space.insert(start + num_blocks, remaining);
                    }
                } else {
                    src -= 1;
                }
            },
        }
    }

    let checksum = day9::compute_checksum(&disk);

    println!("Checksum: {}", checksum);

    Ok(())
}

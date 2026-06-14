use std::{
    io::{self, BufRead},
    iter,
};

pub fn get_disk(
    mut input: impl BufRead, mut free_space_callback: impl FnMut(usize, usize),
) -> io::Result<Vec<i32>> {
    let mut disk_map = vec![];

    input.read_until(b'\n', &mut disk_map)?;

    let mut disk_iter = disk_map.trim_ascii_end().iter();

    let mut disk = vec![];

    let digit = |&val| val - 0x30;

    let mut file_id: i32 = 0;

    loop {
        let Some(file_blocks) = disk_iter.next().map(digit) else {
            break;
        };

        disk.extend(iter::repeat_n(file_id, file_blocks as _));

        file_id += 1;

        let Some(free_blocks) = disk_iter.next().map(digit) else {
            break;
        };

        free_space_callback(disk.len(), free_blocks as usize);

        disk.extend(iter::repeat_n(-1, free_blocks as _));
    }

    Ok(disk)
}

pub fn compute_checksum(disk: &[i32]) -> u64 {
    disk.iter()
        .enumerate()
        .filter(|&(_, &val)| val != -1)
        .map(|(i, &file_id)| i as u64 * file_id as u64)
        .sum()
}

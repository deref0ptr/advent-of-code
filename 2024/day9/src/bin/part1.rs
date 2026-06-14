use std::io;

fn main() -> io::Result<()> {
    let mut disk = day9::get_disk(io::stdin().lock(), |_,_| {})?;

    let mut insertion = 0;
    let mut removal = disk.len() - 1;

    while insertion < removal {
        if disk[insertion] != -1 {
            insertion += 1;
        } else if disk[removal] == -1 {
            removal -= 1;
        } else {
            disk.swap(insertion, removal);
        }
    }

    let checksum = day9::compute_checksum(&disk);

    println!("Checksum: {}", checksum);

    Ok(())
}

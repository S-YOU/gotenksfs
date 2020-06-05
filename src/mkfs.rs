use crate::gotenks::{types::Superblock, util};
use anyhow::anyhow;
use byte_unit::{Byte, ByteUnit};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};

pub fn make<P>(path: P, file_size: u32, blk_size: u32) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let bg_size = util::block_group_size(blk_size);
    if file_size < bg_size - 2 * blk_size {
        return Err(anyhow!(format!(
            "File size must be at least {} for block size of {}",
            Byte::from_bytes(bg_size as _).get_appropriate_unit(true),
            Byte::from_bytes(blk_size as _).get_adjusted_unit(ByteUnit::B)
        )));
    }

    let groups = file_size / bg_size + 1;
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    let mut buf = BufWriter::new(&file);
    let mut sb = Superblock::new(blk_size, groups as _);

    let mut serialized_buf = sb.serialize()?;
    serialized_buf.resize(Superblock::size() as _, 0u8);
    buf.write_all(&serialized_buf)?;

    buf.flush()?;

    Ok(file.set_len(Superblock::size() + bg_size as u64 * groups as u64)?)
}

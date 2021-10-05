// this will let us read integers of various width
use byteorder::{LittleEndian, ReadBytesExt};
use positioned_io::{ReadAt, Cursor, Slice};
use std::fs::OpenOptions;
use anyhow::{Result, anyhow};

struct Reader<IO: ReadAt> {
  inner: IO,
}

#[derive(Debug)]
struct Superblock {
    inode_count: u64,
    block_count: u64,
    su_blocks: u64,
    free_block_count: u64,
    free_inode_count: u64,
    first_data_block: u64,
    block_size: u64,
    cluster_size: u64,
    blocks_per_group: u64,
    clusters_per_group: u64,
    inodes_per_group: u64,
}

impl Superblock {
    fn parse (loc: &dyn ReadAt) -> Result<Superblock> {
	let r = Reader::new(Slice::new(loc, 1024, None));
	if r.u16(0x38)? != 0xEF53 {
	    Err(anyhow!("Magic number is incorrect!"))
	} else {
	    Ok(Superblock {
		inode_count: r.u32(0x0)? as u64,
		block_count: r.u32(0x4)? as u64,
		su_blocks: r.u32(0x8)? as u64,
		free_block_count: r.u32(0xC)? as u64,
		free_inode_count: r.u32(0x10)? as u64,
		first_data_block: r.u32(0x14)? as u64,
		block_size: 2u64.pow(10u32+(r.u32(0x18)?)),
		cluster_size: r.u32(0x1C)? as u64,
		blocks_per_group: r.u32(0x20)? as u64,
		clusters_per_group: r.u32(0x24)? as u64,
		inodes_per_group: r.u32(0x28)? as u64,		
	    })
	}
    }
}

impl<IO: ReadAt> Reader<IO> {
    fn new(inner: IO) -> Self {
        Self { inner }
    }

    fn u16(&self, offset: u64) -> Result<u16> {
        let mut cursor = Cursor::new_pos(&self.inner, offset);
        Ok(cursor.read_u16::<LittleEndian>()?)
    }

    fn u32(&self, offset: u64) -> Result<u32> {
        let mut cursor = Cursor::new_pos(&self.inner, offset);
        Ok(cursor.read_u32::<LittleEndian>()?)
    }

}

// using our new result type:
fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("/dev/sda1")?;
    let s: Superblock = Superblock::parse(&file)?;
    println!("{:#?}", s);
    Ok(())
}

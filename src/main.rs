// // this will let us read integers of various width
// use byteorder::{LittleEndian, ReadBytesExt};
// use positioned_io::{ReadAt, Cursor, Slice};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek, SeekFrom};
use anyhow::{Result, anyhow};

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

enum Element {
	Blank,
	Superblock(Superblock),	
}

struct Reader {
	file: File,
	offset: usize,
}

impl Reader {
	fn new(p: &Path) -> Self {
		Reader {
			file: File::open(p).unwrap(),
			offset: 0,
		}
	}
	
	fn next(&mut self) -> Element {
		if self.offset == 0 {
			if cfg!(debug_assertions) {
				let mut buf: [u8; 1024] = [1; 1024];
				self.file.read(&mut buf);
				print!("Checking if group 0 padding is all 0... ");
				for i in buf {
					assert_eq!(i, 0);
				}
				println!(" check!");
			} else {
				self.file.seek(SeekFrom::Current(1024));
			}
			self.offset += 1024;
			return Element::Blank;
		}
		else if self.offset == 1024 {
			let mut buf = [0; 1024];
			self.file.read(&mut buf);
			print!("Checking if superblock magic number is valid... ");
			assert_eq!(buf[0x38], 0x53);
			assert_eq!(buf[0x39], 0xEF);
			println!(" check!");			
		}
		Element::Blank
	}
}

fn main() {
	let p = Path::new("/dev/nvme0n1p3");
	let mut r = Reader::new(&p);
	r.next();
	r.next();
}

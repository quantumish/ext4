// this will let us read integers of various width
use byteorder::{LittleEndian, ReadBytesExt};
use positioned_io::{ReadAt, Cursor};
use failure::Fallible;
use std::fs::OpenOptions;

type Result<T> = std::result::Result<T, failure::Error>;

struct Reader<IO: ReadAt> {
  inner: IO,
}

impl<IO: ReadAt> Reader<IO> {
    fn new(inner: IO) -> Self {
        Self { inner }
    }

    fn u16(&self, offset: u64) -> Fallible<u16> {
        let mut cursor = Cursor::new_pos(&self.inner, offset);
        Ok(cursor.read_u16::<LittleEndian>()?)
    }
}

use positioned_io::Slice;

// using our new result type:
fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("/dev/sda1")?;

    // create a slice that corresponds to the superblock
    let r = Reader::new(Slice::new(file, 1024, None));

    // as per the docs:
    let magic = r.u16(0x38)?;
    println!("magic = {:x}", magic);

    Ok(())
}

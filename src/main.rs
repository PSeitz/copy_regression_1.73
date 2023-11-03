use std::io::{Read, Result};

struct ChunkReader<'a> {
    data: &'a [u8],
    cursor: usize,
}

impl<'a> ChunkReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        ChunkReader { data, cursor: 0 }
    }
}

impl<'a> Read for ChunkReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let next_chunk = std::cmp::min(buf.len(), self.data.len() - self.cursor).min(32_000);

        buf[..next_chunk].copy_from_slice(&self.data[self.cursor..self.cursor + next_chunk]);
        self.cursor += next_chunk;

        Ok(next_chunk)
    }
}

fn main() {
    let data: Vec<u8> = vec![0; 100_000_000];
    let mut reader = ChunkReader::new(&data);
    let mut out = Vec::new();
    std::io::copy(&mut reader, &mut out).unwrap();
    println!("{}", &out[10]);
}

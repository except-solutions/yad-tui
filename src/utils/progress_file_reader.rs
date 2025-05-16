
pub struct ProgressFileReader<T> {
    pub bytes_readed: usize,
    pub it: T
}

impl<T> ProgressFileReader <T> {}

impl<R: std::io::Read> std::io::Read for ProgressFileReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let inc = self.it.read(buf)?;
        self.bytes_readed += inc;
        Ok(inc)
    }
}


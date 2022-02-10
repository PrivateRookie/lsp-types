use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct BytesCodec<S: Read + Write> {
    stream: S,
}

impl<S: Read + Write> BytesCodec<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

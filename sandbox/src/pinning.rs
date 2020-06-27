// Source: https://www.youtube.com/watch?v=DkMwYxfSYNQ&t=1553s

async fn foo(s: &mut dyn AsyncRead + AsyncWrite) {

}

struct IoRead<'a> {
    buf : &'a mmut [u8],
    s: &mut dyn AsyncRead
}

impl<'a> Future for IoRead<'a> {
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self) -> Poll<Self::Output> {

    }
}

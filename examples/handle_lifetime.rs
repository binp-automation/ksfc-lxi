struct A<'a>(&'a str);
struct B<'a, 'b> where 'a: 'b {
    a: &'b mut A<'a>,
}
struct C<'a, 'b> where 'a: 'b {
    a: &'b mut A<'a>,
}

impl<'a> A<'a> {
    fn b<'b>(&'b mut self) -> B<'a, 'b> where 'a: 'b {
        B { a: self }
    }
    fn c<'b>(&'b mut self) -> C<'a, 'b> where 'a: 'b {
        C { a: self }
    }
}

impl<'a, 'b> B<'a, 'b> where 'a: 'b {
    fn call(&mut self) {
        (|| {
            self.a.c().call();
        })();
    }
}
impl<'a, 'b> C<'a, 'b> where 'a: 'b {
    fn call(&mut self) {
        println!("{}", self.a.0);
    }
}


fn main() {
    let mut a = A("hello");
    let mut b = a.b();

    b.call();
}

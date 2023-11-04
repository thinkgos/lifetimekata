fn main() {}

trait Foo {}
impl Foo for &'_ str {}

fn f1<T: Foo>(t: T) -> Box<T> {
    Box::new(t)
}

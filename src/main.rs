fn main() {
    main1();
    main2();
}
struct Fibonacci {
    a: u128,
    b: u128,
}
impl Iterator for Fibonacci {
	type Item = u128;
	fn next(&mut self) -> Option<u128> {
		let new_b = self.a + self.b;
		self.a = self.b;
		self.b = new_b;
		return Some(self.a)
	}
}
fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}
fn main1() {
    for number in fibonacci_numbers() {
        if number < 205697230343233228174223751303346572685 {
            println!("{}", number);
        }else {
            println!("Overflow!");
            break;
        }
    }
}
struct StrDisplayable<'a>(Vec<&'a str>);
impl fmt::Display for StrDisplayable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}
use std::fmt;
fn main2() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_Foo = StrDisplayable(vec);
        println!("{}",vec_Foo);
}
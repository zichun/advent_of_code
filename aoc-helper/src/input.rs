pub use std::str::FromStr;

pub trait TokenReader {
    fn next_token<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug;
}
impl<'a, I: Iterator<Item = &'a str>> TokenReader for I {
    fn next_token<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.next().unwrap().parse::<T>().unwrap()
    }
}
pub trait InputHelper<'a>
where
    Self: Sized,
{
    fn extract_tokens<T: FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a>;
    fn parse_tokens<T: FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a>
    where
        <T as FromStr>::Err: std::fmt::Debug;
}
impl<'a> InputHelper<'a> for &'a str {
    fn extract_tokens<T: FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a> {
        Box::new(self.split_whitespace().filter_map(|t| t.parse::<T>().ok()))
    }

    fn parse_tokens<T: FromStr>(&self) -> Box<dyn Iterator<Item = T> + 'a>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        Box::new(self.split_whitespace().map(|t| t.parse::<T>().unwrap()))
    }
}

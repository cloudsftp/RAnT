use rant::simulate::find_cycle;

fn main() {
    let a = 3.1;
    let f = |x: f64| a * x * (1f64 - x);

    println!("{:?}", find_cycle(f, 0.5, 1_000, 10))
}

#[cfg(test)]
mod test {
    #[test]
    fn foo_test() {
        assert!(false);
    }
}

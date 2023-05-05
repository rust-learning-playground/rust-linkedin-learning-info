pub fn info<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_str_slice() {
        info("msg");
    }

    #[test]
    fn print_str() {
        info(String::from("msg"));
    }
}

struct Dial {
    position: i32,
}

impl Dial {
    pub fn new() -> Self {
        Dial { position: 50 }
    }
}

#[cfg(test)]
mod tests {
    use super::Dial;

    #[test]
    fn test_dial() {
        let dial = Dial::new();

        assert_eq!(dial.position, 50);
    }
}

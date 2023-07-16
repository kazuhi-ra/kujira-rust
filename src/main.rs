struct User {
    name: String,
    age: u8,
}

#[cfg(test)]
mod tests {
    use super::User;
    #[test]
    fn strc() {
        let u1 = User {
            name: "kazuhira".to_string(),
            age: 27,
        };
        let u2 = User {
            name: "kirito".to_string(),
            age: 16,
        };

        assert_eq!(u1, u2)
    }
}

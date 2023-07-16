fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn array() {
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2)
    }

    #[test]
    fn vec() {
        let v1: Vec<u16> = vec![100, 200, 300];
        let v2: Vec<u16> = vec![100, 200, 300];
        assert_eq!(v1, v2)
    }
}

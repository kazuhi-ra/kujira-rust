use std::collections::HashMap;

const V_DATA: &str = "C,A,A,B,C,A,A,B,C,A,A,B,C,A,A,B,C,A,A,B,C,A,A,B,U";

fn main() {
    let mut c_map: HashMap<&str, u8> = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);

    for w in V_DATA.split(',') {
        match c_map.get(w) {
            None => println!("unko: {}", w),
            Some(v) => {
                c_map.insert(w, *v + 1);
            }
        }
    }

    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}

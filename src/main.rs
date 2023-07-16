trait TreasureBox {
    fn opne(&self, key_no: u32) -> bool;
    fn check(&self);
}

struct JewelryBox {
    price: u32,
    key_no: u32,
}

impl TreasureBox for JewelryBox {
    fn opne(&self, key_no: u32) -> bool {
        self.key_no == key_no
    }

    fn check(&self) {
        println!("price: {}", self.price)
    }
}

struct TrapBox {
    damege: u32,
}

impl TreasureBox for TrapBox {
    fn opne(&self, _: u32) -> bool {
        true
    }

    fn check(&self) {
        println!("damage: {}", self.damege)
    }
}

struct Key {
    number: u32,
}

struct User {
    name: String,
    key: Key,
}

trait BoxOpener {
    fn open_box(&self, tbox: impl TreasureBox);
}

impl BoxOpener for User {
    fn open_box(&self, tbox: impl TreasureBox) {
        if !tbox.opne(self.key.number) {
            println!("{} can't open box", &self.name);
            return;
        }
        tbox.check();
    }
}

fn main() {
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = TrapBox { damege: 10 };
    let my_key = Key { number: 2 };
    let kirito = User {
        name: "kirito".to_string(),
        key: my_key,
    };

    kirito.open_box(box1);
    kirito.open_box(box2);
}

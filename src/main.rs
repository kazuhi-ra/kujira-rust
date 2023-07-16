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

fn open_box(tbox: impl TreasureBox, key_no: u32) {
    if !tbox.opne(key_no) {
        println!("can't open");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = TrapBox { damege: 10 };

    let my_key = 2;
    open_box(box1, my_key);
    open_box(box2, my_key);
}

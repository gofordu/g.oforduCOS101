struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    let hp_qty = 3;
    let ibm_qty = 3;
    let toshiba_qty = 3;
    let dell_qty = 3;

    println!("{} total: {}", hp.brand, hp.cost(hp_qty));
    println!("{} total: {}", ibm.brand, ibm.cost(ibm_qty));
    println!("{} total: {}", toshiba.brand, toshiba.cost(toshiba_qty));
    println!("{} total: {}", dell.brand, dell.cost(dell_qty));

    let total = hp.cost(hp_qty) + ibm.cost(ibm_qty) + toshiba.cost(toshiba_qty) + dell.cost(dell_qty);

    println!("Grand total: {}", total);
}
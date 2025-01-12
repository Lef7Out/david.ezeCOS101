
struct Laptop {
    brand: String,
    price: u64,
    available: u64,
}

impl Laptop {
    fn total_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
        available: 10,

    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
        available: 6,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
        available: 10,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
        available: 4,
    };

    
    let quantity = 3;

    
    let total_hp = hp.total_cost(quantity);
    let total_ibm = ibm.total_cost(quantity);
    let total_toshiba = toshiba.total_cost(quantity);
    let total_dell = dell.total_cost(quantity);

    
    let overall_total = total_hp + total_ibm + total_toshiba + total_dell;

    
    println!("Total cost for {} HP laptops: ₦{}", quantity, total_hp);
    println!("Total cost for {} IBM laptops: ₦{}", quantity, total_ibm);
    println!("Total cost for {} Toshiba laptops: ₦{}", quantity, total_toshiba);
    println!("Total cost for {} Dell laptops: ₦{}", quantity, total_dell);
    println!("Overall total cost: ₦{}", overall_total);
}

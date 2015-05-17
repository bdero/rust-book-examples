struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}

fn main() {
    let p1 = Philosopher::new("Frantz Fanon");
    let p2 = Philosopher::new("Thomas Sankara");
    let p3 = Philosopher::new("Maria Mies");
    let p4 = Philosopher::new("Che Guevara");
    let p5 = Philosopher::new("Vladimir Lenin");
    let p6 = Philosopher::new("Zak Cope");
}

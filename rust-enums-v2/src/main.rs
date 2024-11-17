enum Colours {
    Red,
    Green,
    Blue,
    Yellow,
}

struct Person {
    age: i32,
    name: String,
}

struct Employee {
    person: Person,
    job: String,
}

impl Employee {
    fn print(&self) {
        println!("Hi I am {} and I work as a {}.", self.person.name, self.job);
    }

    fn append(&mut self) {
        self.person.name.push_str("Sloths");
    }
}

impl Colours {
    pub fn print(&self) {
        match self {
            Colours::Red => println!("Red"),
            Colours::Green => println!("Green"),
            Colours::Blue => println!("Blue"),
            _ => println!("Yellow"),
        };
    }

    fn is_green(&self) -> bool {
        if let Colours::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Colours::Red => false,
            Colours::Green => true,
            Colours::Blue => true,
            Colours::Yellow => true,
        }
    }
}

fn append(employees: &mut Vec<Employee>) {
    for emp in employees {
        emp.append();
    }
}

fn main() {
    let colour: Colours = Colours::Red;
    colour.print();

    let colour2: Colours = Colours::Green;
    colour2.print();

    let colour3: Colours = Colours::Blue;
    colour3.print();

    let colour4: Colours = Colours::Yellow;
    colour4.print();

    let mut employees: Vec<Employee> = vec![];
    append(&mut employees);
}

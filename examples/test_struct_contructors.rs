struct Droppable {
    name: &'static str,
    int_value: u16,
}


impl Droppable {
    fn function_first(&mut self) -> u16
    {
        //10
        self.int_value
    }
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    println!("start of the main function");
    println!("Inserting block A");

    let _a = Droppable { name: "A",int_value: 10 };

    // block A
    {
        let _b = Droppable { name: "B" , int_value: 20};

        // block B
        {

            println!("Inserting block B");

            let mut _c = Droppable { name: "C",int_value: 10 };

            println!("_c.function_first() => {:#?}",_c.function_first());

            let mut  _d = Droppable { name: "D",int_value: 20 };

            println!("_d.function_first() => {:#?}",_d.function_first());

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}

#[test]

/*
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            __::Add => x + y,
            __::Subtract => x - y,
        }
    }
}
*/
fn main() {
    let add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let subtract = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;

    println!("Add: {}", add.run(5, 3)); // Output: 8
    println!("Subtract: {}", subtract.run(5, 3)); // Output: 2
}
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
            VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
        }
    }
}




/*
У match виразі потрібно вказати повне ім'я перерахунку,
тобто VeryVerboseEnumOfThingsToDoWithNumbers::Add і VeryVerboseEnumOfThingsToDoWithNumbers::Subtract,
щоб розпізнати кожен варіант перерахунку.

Функція run: В залежності від того, який варіант перерахунку був переданий, функція виконує додавання або віднімання між x та y.
*/
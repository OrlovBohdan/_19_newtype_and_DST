#[test]

/*
use std::fmt;

/* Define the Wrapper type */
__;

// Display is an external trait
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // Vec is an external type, so you cannot implement Display trait on Vec type
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
*/
fn main() {
    // Vec is an external type, so you cannot implement Display trait on Vec type
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
use std::fmt;

// Define the Wrapper type
struct Wrapper(Vec<String>);

// Display is an external trait
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}




/*
struct Wrapper(Vec<String>);: Ми оголошуємо структуру Wrapper, яка містить один елемент типу Vec<String>.
Це дозволяє використовувати її для обгортання вектора рядків.

impl fmt::Display for Wrapper: Реалізація трейту Display для типу Wrapper.
У методі fmt ми використовуємо метод join, щоб об'єднати елементи вектора в один рядок, розділений комами,
і обгортаємо результат в квадратні дужки.
*/
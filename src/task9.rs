#[test]

/*
/* Make it work in two ways */
use std::fmt::Display;
fn foobar(thing: Display) {}

fn main() {
}
*/

//1

fn main() {
    let s = String::from("Hello, world!");
    let num = 42;

    // Викликаємо функцію з різними типами
    foobar(s);
    foobar(num);
}
use std::fmt::Display;

fn foobar(thing: impl Display) {  // Використовуємо `impl Display` для узагальненого типу
    println!("{}", thing); // Друкуємо значення, реалізуючи Display
}



//2
/*fn main() {
    let s = String::from("Hello, world!");
    let num = 42;

    // Викликаємо функцію з конкретними типами
    foobar(s);
    foobar(num);
}
use std::fmt::Display;

fn foobar<T: Display>(thing: T) {  // `T: Display` це обмеження для типу T
    println!("{}", thing); // Друкуємо значення
}*/



/*
//1
Передача значення безпосередньо
Передати тип, який реалізує Display, наприклад, String або i32.

//2
2. Використання конкретного типу, що реалізує Display
Можна також використовувати конкретний тип, що реалізує Display, наприклад String або i32, і передавати їх функції.

Використання impl Display: У функції foobar можно використати параметр типу impl Display,
що дозволяє передавати будь-який тип, який реалізує трейт Display.

Використання обмеження типу через <T: Display>: Цей варіант дозволяє створити узагальнену функцію,
яка приймає будь-який тип, що реалізує трейт Display.

*/
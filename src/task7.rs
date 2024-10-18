#[test]

/*
/* Make it work with const generics */
fn my_function(n: usize) -> [u32; usize] {
    [123; n]
}

fn main() {
    let arr = my_function();
    println!("{:?}",arr);
}
*/
fn main() {
    let arr = my_function::<5>(); // Specify the size of the array as a constant
    println!("{:?}", arr);
}

// Use const generics
fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}

/*
Const Generics: Використовуємо const N: usize як параметр типу для функції my_function.
Тепер можна передавати розмір масиву як константу при виклику функції.
Виклик функції: Для того щоб викликати my_function, потрібно вказати розмір масиву,
як константу, наприклад, my_function::<5>(), де 5 — це розмір масиву.
*/
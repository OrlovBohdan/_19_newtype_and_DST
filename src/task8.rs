#[test]

/*
/* Make it work with slice references */
fn main() {
    let s: str = "Hello there!";

    let arr: [u8] = [1, 2, 3];
}
*/

fn main() {
    // `&str` — слайс рядка
    let s: &str = "Hello there!";

    // Створення масиву типу `u8`
    let arr: [u8; 3] = [1, 2, 3];

    // Створення слайса з масиву
    let slice = &arr[1..]; // Слайс, що містить елементи з 1 індексу до кінця

    // Виведення значень
    println!("String slice: {}", s);
    println!("Array slice: {:?}", slice);
}


/*
let s: &str = "Hello there!"; — тут змінна s є слайсом рядка, який посилається на літеральний рядок "Hello there!".
let arr: [u8; 3] = [1, 2, 3]; — масив типу u8 з трьома елементами.
let slice = &arr[1..]; — слайс масиву arr починається з індексу 1 і містить елементи до кінця масиву.
*/
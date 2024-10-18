#[test]

/*
/* Make it workd */
struct Meters(u32);

fn main() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    // The `pow` method is defined on `u32` type, we can't directly call it
    assert_eq!(n.pow(2), 4);
}
*/
fn main() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    // Тепер ми можемо викликати pow для Meters
    assert_eq!(n.pow(2), 4);
}
struct Meters(u32);

impl Meters {
    // Реалізуємо метод pow для типу Meters
    fn pow(&self, exponent: u32) -> u32 {
        self.0.pow(exponent) // викликаємо pow для внутрішнього значення
    }
}




/*
impl Meters: Реалізація методу pow для структури Meters. У цьому методі ми викликаємо метод pow для внутрішнього
значення self.0, яке є типом u32.

self.0.pow(exponent): self.0 — це доступ до значення типу u32 всередині структури Meters.
Тепер ви можете викликати метод pow на цьому значенні.
*/
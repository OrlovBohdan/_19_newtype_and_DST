#[test]

/*
use std::ops::Add;
use std::fmt::{self, format};

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}
fn main() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}",d), "There are still 30 meters left");
}

/* Implement calculate_distance  */
fn calculate_distance
*/
fn main() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}",d), "There are still 30 meters left");
}
use std::ops::Add;
use std::fmt::{self};

struct Meters(u32);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}



/* Implement calculate_distance */
fn calculate_distance(m1: Meters, m2: Meters) -> Meters {
    m1 + m2
}


/*
Функція calculate_distance: Вона приймає два аргументи типу Meters,
додає їх за допомогою перевантаженого оператора + (за допомогою реалізації трейтів Add), і повертає новий об'єкт типу Meters.

Додавання Meters: Оскільки ми вже реалізували трейт Add для типу Meters, операція додавання працює
як передбачено, і в результаті ми отримуємо новий об'єкт типу Meters, що містить суму значень обох аргументів.
*/
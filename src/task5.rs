#[test]

/*
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

/* Fill in the blank */
__

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}
*/
fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let _x = Operations::Add;
}

#[allow(dead_code)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Define alias for the enum
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;




/*
Alias (псевдонім): Використовується ключове слово type для створення псевдоніма. Тепер ви можете використовувати Operations замість довгого імені VeryVerboseEnumOfThingsToDoWithNumbers.

Використання в main: Ви можете тепер використовувати Operations::Add для доступу до варіанту перерахунку Add.
*/
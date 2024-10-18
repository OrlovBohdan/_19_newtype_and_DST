#[test]

/*
/* Make it work */
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

// An age verification function that checks age in years, must be given a value of type Years.
fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days));
}
*/
fn main() {
    let age = Years(5);
    let age_days = age.to_days();

    // Now we convert age_days into Years before passing it to old_enough
    let age_from_days = age_days.to_years();

    println!("Old enough in years: {}", old_enough(&age));
    println!("Old enough in days: {}", old_enough(&age_from_days));
}
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

// An age verification function that checks age in years, must be given a value of type Years.
fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}




/*
old_enough: Ця функція тепер правильно приймає параметр типу &Years, тобто перевірка віку відбувається на основі років.

Конвертація Days у Years: Щоб передати Days у функцію old_enough, ми додаємо метод to_years для конвертації кількості днів у роки
*/

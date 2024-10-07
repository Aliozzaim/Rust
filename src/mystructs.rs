pub struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
}

impl Person {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn birth_date(&self) -> (u16, u8) {
        (self.birth_year, self.birth_month)
    }
}

pub fn new_person(
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
) -> Person {
    let p1 = Person {
        first_name,
        last_name,
        birth_year,
        birth_month,
    };
    return p1;
}

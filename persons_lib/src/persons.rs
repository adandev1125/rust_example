#[derive(Debug)]
pub enum Gender {
    Male, Female
}

pub trait Walkable {
    fn walk(&self);
}

pub struct Person {
    name: String,
    gender: Gender,
    age: i32,
}
impl Person {
    pub fn display(&self) {
        assert!(self.age > 0);
        println!("Hello world, {}({:?}, {})", self.name, self.gender, self.age);
    }

    pub fn create_person(name: &str, gender: Gender, age: i32) -> Person {
        return Person {name: String::from(name), gender, age};
    }

    pub fn age(&self) -> i32 {
        return self.age;
    }

    pub fn increase_age(&mut self) {
        self.age += 1;
    }
}
impl Walkable for Person {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}
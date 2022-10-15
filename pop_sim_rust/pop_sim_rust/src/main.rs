use std::str;
use std::time::Instant;
use rand::Rng;


// Person data struct
#[derive(Debug)]
pub struct Person {
    id: i64,
    name: &'static str,
    gender: i16,
    age: i32,
    love_vec: Vec<i64>,
}

fn main() {
    static mut POPULATION: i64 = -1;
    static mut PEOPLE: Vec<Person> = Vec::new();

    pub fn create_person() -> Person {
        unsafe { POPULATION += 1 };
        let temp_person: Person = Person {
            id: unsafe { POPULATION },
            name: "John",
            gender: 0,
            age: 0,
            love_vec: vec![-1, 100],
        };

        return temp_person;
    }

    pub fn update_sim(mut steps: i32) -> i32 {
        let people_temp = unsafe { &mut PEOPLE };

        for id in 0..unsafe { PEOPLE.len() as usize } {
            // Ages all people by 1 month
            // println!("{:?}", people_temp);
            people_temp[id].age += 1;

            if people_temp[id].love_vec[0] == -1 {
                // Creates a random number to chose a lover for person
                let lover = rand::thread_rng().gen_range(0..=(unsafe { PEOPLE.len() } - 1)) as i64;

                // If the person is not the lover and if the person does not have a lover one is given
                if lover != id as i64 && people_temp[id].love_vec[0] == -1 {
                    people_temp[id].love_vec[0] = lover;
                    steps += 1;
                }
                steps += 1;
            }


            if people_temp[id].love_vec[1] as i32 != -1 {
                let baby_chance = rand::thread_rng().gen_range(0..100) as i32;
                if baby_chance < 2 {
                    // Creates a baby!!!
                    let people_temp = unsafe { &mut PEOPLE };
                    let john: Person = create_person();
                    people_temp.push(john);
                    steps += 1;
                }
            }

            if people_temp[id].age > 12 * 30 {
                unsafe { PEOPLE.remove(id); }
            }
            steps += 1;
        }
        steps
    }

    pub fn print_people() {
        println!("\n**~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~**");
        for id in 0..unsafe { PEOPLE.len() } {
            println!("------------------------------------------");
            unsafe {
                println!("[ID: {:?}]\n\
                  Name: {:?}\n\
                  Age: {:?}\n\
                  Gender: {:?}\n\
                  Lover: {:?}", PEOPLE[id].id, PEOPLE[id].name, PEOPLE[id].age,
                         PEOPLE[id].gender, PEOPLE[id].love_vec)
            }
        }
    }

    let start = Instant::now();

    let people_temp = unsafe { &mut PEOPLE };

    let john: Person = create_person();
    people_temp.push(john);

    let john2: Person = create_person();
    people_temp.push(john2);

    // Graphing variables
    // let mp: Vec<i32> = Vec::new();
    // let pop: Vec<i32> = Vec::new();
    // let tp: i32 = -1;

    print_people();
    let mut steps = 0;

    for _ in 0..12 * 100 {
        steps = update_sim(steps);
    }

    let duration = start.elapsed();

    println!("\nPeople: {:?} | Steps: {}", people_temp.len(), steps);

    // Time took to complete code
    println!("Time taken to calculate: {:?}", duration);
    // print_people();
}

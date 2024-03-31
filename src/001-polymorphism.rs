// 1)
// traits allow polymorphism
// traits allow abstract methods
// impl traitName for structName {}

// 2)
// let say we have a trait Draw
// allow properties can be accessed by the trait by Box<dyn traitName>
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// 3)
// Vector can't store different types of data
// but it can store different types of structs that implement the same trait
// with Box<dyn traitName>
// Vector of Boxes (smart pointers)

// 4) dynamic dispatch
// Rust uses static dispatch by default
// Rust uses dynamic dispatch when using trait objects
// Rust uses dynamic dispatch when using Box<dyn traitName>
// Example
// pub trait Draw {
//     fn draw(&self);
// }
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }
// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//            // Dynamic dispatch, only call at runtime
//            // because at compile time, Rust doesn't know what type of component is
//            // but it knows that the component implements the Draw trait
//             component.draw();
//         }
//     }
// }

mod polygui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn render(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("*** button: {} ** \n", self.label);
        }
    }

    pub struct ListBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for ListBox {
        fn draw(&self) {
            println!("--- listbox ---");
            for option in self.options.iter() {
                println!("*** option: {} **", option);
            }
            print!("")
        }
    }
}

use polygui::{Button, ListBox, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(ListBox {
                width: 100,
                height: 200,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
        ],
    };

    screen.render();
}

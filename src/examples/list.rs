#![allow(dead_code)]
use std::fmt::format;

use crate::List::*;

#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new () -> List {
        Nil
    }

    fn prepend (self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn append (self, elem: u32) -> List {
        match self {
            List::Cons(head, tail) => List::Cons(head, Box::new(tail.append(elem))),
            List::Nil => List::Cons(elem, Box::new(List::Nil))
        }
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main()
{    
    let mut list = List::new();
    
    list = list.append(1);
    list = list.append(2);
    list = list.prepend(3);

    println!("The len is {}", list.len());
    println!("{}", list.stringify());
}
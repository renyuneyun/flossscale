#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod question;

use std::io;
use std::fmt;

use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, FromForm};
use rocket::response::Redirect;

use rocket_contrib::Template;

use question::{Axis, Axes};
use question::{Choice, Question, Questions};

#[derive(Serialize)]
struct Mark {
    axes: Axis,
    marks: Vec<i32>,
}

impl Mark {
    fn from(ax: &Axis) -> Mark {
        let mut marks = Vec::new();
        for a in ax {
            marks.push(0);
        }
        Mark {
            axes: ax.clone(),
            marks: marks,
        }
    }
}

#[derive(Serialize)]
struct Marks {
    marks: Vec<Mark>,
}

impl Marks {
    fn from(axis: &Axes) -> Marks {
        let mut marks = Vec::new();
        for a in axis {
            marks.push(Mark::from(a));
        }
        Marks {
            marks: marks
        }
    }

    fn mut_mark_of(&mut self, target: &String) -> Option<&mut i32> {
        for mark in &mut self.marks {
            let axis: &Axis = &mark.axes;
            for i in 0 .. axis.len() {
                let value: &String = axis.get(i).unwrap();
                if *value == *target {
                    return mark.marks.get_mut(i)
                }
            }
        }
        None
    }

    fn add_for(&mut self, target: &String, inc: i32) {
        if let Some(mark) = self.mut_mark_of(target) {
            *mark += inc;
        } else {
            panic!();
        }
    }
}

#[derive(FromForm)]
struct UserSelection {
    selection: usize,
}

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context<'a> {
        name: &'a str,
    };
    let context = Context {
        name: "asd",
    };
    Template::render("index", &context)
}

#[get("/<id>")]
fn question(id: usize, questions: State<Questions>) -> Template {
    let question: &Question = questions.get(id).expect("out of index");

    #[derive(Serialize)]
    struct PureQuestion<'a> {
        id: usize,
        question_text: &'a str,
        choices: Vec<&'a str>,
    };
    let mut choices: Vec<&str> = Vec::new();
    for c in &question.choices {
        choices.push(&c.text);
    }
    let pquestion = PureQuestion {
        id: id,
        question_text: &question.text,
        choices: choices,
    };
    Template::render("question", &pquestion)
}

#[post("/<id>", data = "<choice>")]
fn answer(id: usize, choice: Form<UserSelection>, mut cookies: Cookies, questions: State<Questions>) -> Redirect {
    let cc = Cookie::new(id.to_string(), choice.get().selection.to_string());
    cookies.add(cc);
    match id + 1 < questions.len() {
        true => {
            let red_addr: String = "/".to_owned() + &(id+1).to_string();
            Redirect::to(&red_addr)
        },
        false => {
            Redirect::to("/result")
        },
    }
}

#[get("/result")]
fn result(cookies: Cookies, questions: State<Questions>) -> Template {
    let axes = question::dummy_axes();
    let mut marks: Marks = Marks::from(&axes);

    for i in 0 .. questions.len() {
        let q = questions.get(i).unwrap();
        let cc = cookies.get(&i.to_string()).expect("Cookie doesn't have this item");
        let ans: usize = cc.value().parse::<usize>().unwrap();
        for (axis, sc) in &q.choices.get(ans).expect("Choice not in available answers").scores {
            marks.add_for(axis, *sc);
        }
    }

    #[derive(Serialize)]
    struct IndScore {
        name: String,
        score: i32,
    };
    #[derive(Serialize)]
    struct Context {
        items: Vec<IndScore>,
    };
    let mut context = Context{
        items: Vec::new(),
    };
    for mark in &marks.marks {
        for i in 0 .. mark.axes.len() {
            context.items.push(IndScore {name: mark.axes.get(i).unwrap().clone(), score: *mark.marks.get(i).unwrap()})
        }
    }
    Template::render("result", &context)
}

fn main() {
    let questions = question::questions();

    rocket::ignite()
        .mount("/", routes![index, question, answer, result])
        .attach(Template::fairing())
        .manage(questions)
        .launch();
}

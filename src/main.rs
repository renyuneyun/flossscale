#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io;
use std::fmt;

use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, FromForm};
use rocket::response::Redirect;

use rocket_contrib::Template;

struct Choice<'a> {
    text: &'a str,
    scores: Vec<(String, i32)>,
}

struct Question<'a> {
    text: &'a str,
    choices: Vec<Choice<'a>>,
}

type Questions<'a> = Vec<Question<'a>>;

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
    let mut choices = Vec::new();
    for c in &question.choices {
        choices.push(c.text);
    }
    let pquestion = PureQuestion {
        id: id,
        question_text: question.text,
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
    type Scores = HashMap<String, i32>;
    
    let mut scores: Scores = HashMap::new();
    for i in 0 .. questions.len() {
        let q = questions.get(i).unwrap();
        let cc = cookies.get(&i.to_string()).expect("Cookie doesn't have this item");
        let ans: usize = cc.value().parse::<usize>().unwrap();
        for (axis, sc) in &q.choices.get(ans).expect("Choice not in available answers").scores {
            let entry: Entry<String, i32> = scores.entry(axis.clone());
            match entry {
                Occupied(mut ent) => {
                    *ent.get_mut() += sc;
                },
                Vacant(mut ent) => {
                    ent.insert(*sc);
                },
            }
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
    for (k, v) in &scores {
        context.items.push(IndScore {name: k.to_string(), score: *v});
    }
    Template::render("result", &context)
}

fn main() {
    let mut questions: Vec<Question> = Vec::new();
    let q1 = Question {
        text: "Do you think FLOSS should protect themselves?",
        choices: vec![
            Choice {
                text: "yes",
                scores: vec![(String::from("GNU"), 1)],
            },
            Choice {
                text: "no",
                scores: vec![(String::from("OSS"), 1)],
            },
            Choice {
                text: "why floss",
                scores: vec![(String::from("Proprietary"), 1)],
            },
        ],
    };
    let q2 = Question {
        text: "lolololo",
        choices: vec![
            Choice {
                text: "wow",
                scores: vec![(String::from("FLOSS"), 0)],
            },
        ],
    };
    questions.push(q1);
    questions.push(q2);

    rocket::ignite()
        .mount("/", routes![index, question, answer, result])
        .attach(Template::fairing())
        .manage(questions)
        .launch();
}

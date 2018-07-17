#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::io;
use std::fmt;

use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, FromForm};
use rocket::response::Redirect;

use rocket_contrib::Template;

#[derive(Debug)]
enum Axis {
    GNU,
    OSS,
    Proprietary,
}

//impl Axis {
//    fn text(&self) -> String {
//        match self {
//            Axis::GNU => "GNU".to_string(),
//            Axis::OSS => "OSS".to_string(),
//            Axis::Proprietary => "Proprietary".to_string(),
//        }
//    }
//}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            //Axis::GNU => "GNU".to_string(),
            //Axis::OSS => "OSS".to_string(),
            //Axis::Proprietary => "Proprietary".to_string(),
            Axis::GNU => write!(f, "GNU"),
            Axis::OSS => write!(f, "OSS"),
            Axis::Proprietary => write!(f, "Proprietary"),
        }
    }
}

struct Choice {
    text: &'static str,
    scores: Vec<(Axis, i32)>,
}

struct Question {
    text: &'static str,
    choices: Vec<Choice>,
}

type Questions = Vec<Question>;

#[derive(FromForm)]
struct UserSelection {
    selection: usize,
}

fn page_of_question(question: &Question) -> String {
    let mut text: String = question.text.to_string() + "\n";
    for choice in &question.choices {
        text += choice.text;
        text += "\n";
    }
    text
}

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        name: &'static str,
    };
    let context = Context {
        name: "asd",
    };
    Template::render("index", &context)
}

#[get("/<id>")]
fn question(id: usize, questions: State<Questions>) -> String {
    let question: &Question = questions.get(id).expect("out of index");
    page_of_question(question)
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
    let mut scores = [0, 0, 0];
    for i in 0 .. questions.len() {
        let q = questions.get(i).unwrap();
        let cc = cookies.get(&i.to_string()).expect("Cookie doesn't have this item");
        let ans: usize = cc.value().parse::<usize>().unwrap();
        for (axis, sc) in &q.choices.get(ans).expect("Choice not in available answers").scores {
            match axis {
                Axis::GNU => scores[0] += sc,
                Axis::OSS => scores[1] += sc,
                Axis::Proprietary => scores[2] += sc,
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
    let ord = [Axis::GNU, Axis::OSS, Axis::Proprietary];
    for i in 0 .. ord.len() {
        context.items.push(IndScore {name: format!("{}", ord[i]), score: scores[i]});
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
                scores: vec![(Axis::GNU, 1)],
            },
            Choice {
                text: "no",
                scores: vec![(Axis::OSS, 1)],
            },
            Choice {
                text: "why floss",
                scores: vec![(Axis::Proprietary, 1)],
            },
        ],
    };
    questions.push(q1);

    rocket::ignite()
        .mount("/", routes![index, question, answer, result])
        .attach(Template::fairing())
        .manage(questions)
        .launch();
}

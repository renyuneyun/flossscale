pub type Axis = Vec<String>;
pub type Axes = Vec<Axis>;

pub struct Choice {
    pub text: String,
    pub scores: Vec<(String, i32)>,
}

pub struct Question {
    pub text: String,
    pub choices: Vec<Choice>,
}

pub type Questions = Vec<Question>;

pub fn dummy_axes() -> Axes {
    let mut axes: Axes = Vec::new();
    axes.push(vec![String::from("GNU"), String::from("OSS")]);
    axes.push(vec![String::from("FLOSS"), String::from("Proprietary")]);
    axes
}

fn dummy_questions() -> Questions {
    let mut questions: Vec<Question> = Vec::new();
    let q1 = Question {
        text: String::from("Do you think FLOSS should protect themselves?"),
        choices: vec![
            Choice {
                text: String::from("yes"),
                scores: vec![(String::from("GNU"), 1)],
            },
            Choice {
                text: String::from("no"),
                scores: vec![(String::from("OSS"), 1)],
            },
            Choice {
                text: String::from("why floss"),
                scores: vec![(String::from("Proprietary"), 1)],
            },
        ],
    };
    let q2 = Question {
        text: String::from("lolololo"),
        choices: vec![
            Choice {
                text: String::from("wow"),
                scores: vec![(String::from("FLOSS"), 0)],
            },
        ],
    };
    questions.push(q1);
    questions.push(q2);

    questions
}

pub fn questions() -> Questions {
    dummy_questions()
}

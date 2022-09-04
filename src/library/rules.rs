use std::{collections::HashMap, fmt::format};

use crate::Class;

// struct Object<'a>(HashMap<&'a str, &'a str>);
// struct CallBack(Box<dyn Fn(&Vec<&str>) -> String>);
pub enum Style<'a> {
    Object(HashMap<&'a str, &'a str>),
    CallBack(Box<dyn Fn(&Vec<&str>) -> Vec<String>>),
}

impl<'a> Style<'a> {
    fn mapped<const T: usize>(array: [(&'a str, &'a str); T]) -> Style<'a> {
        let mut map = HashMap::new();
        for (rule, value) in array {
            map.insert(rule, value);
        }

        Style::Object(map)
    }
}

pub struct Rule<'a> {
    matcher: &'a str,
    allow_param_tovalue: bool,
    pub styles: Style<'a>,
}

pub struct Rules<'a> {
    pub rules: HashMap<&'a str, Rule<'a>>,
}

impl<'a> Rules<'a> {
    pub fn mapped() -> HashMap<&'a str,Rule<'a>> {
        let mut rules_maped = HashMap::new();

        let rules = Rules::get();

        for rule in rules {
            rules_maped.insert(rule.matcher, rule);
        }

        rules_maped
    }

    fn get() -> Vec<Rule<'a>> {
        vec![
            Rule {
                matcher: "Px",
                allow_param_tovalue: true,
                styles: Style::mapped([("padding-left", "${0}"), ("padding-right", "${0}")]),
            },
            Rule {
                matcher: "Fz",
                allow_param_tovalue: true,
                styles: Style::mapped([("font-size", "${0}")]),
            },
            Rule {
                matcher: "Bgc",
                allow_param_tovalue: true,
                styles: Style::CallBack(Box::new(|args| {
                    vec![format!("background-color: {}", args[0])]
                })),
            },
        ]
    }
}

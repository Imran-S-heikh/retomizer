use std::collections::HashMap;

use crate::Class;

pub enum Style<'a> {
    HashMap(HashMap<&'a str, &'a str>),
    CallBack(Box<dyn Fn(&Vec<&str>) -> String>),
}

pub struct Rule<'a> {
    matcher: &'a str,
    allow_param_tovalue: bool,
    pub styles: Style<'a>,
}

pub struct Rules<'a> {
    pub rules: HashMap<&'a str, Rule<'a>>,
}

fn handle_styles(this: &Rule, class: Class) {}

impl<'a> Rules<'a> {
    pub fn new() -> Rules<'a> {
        let options = [("padding-left", "${0}"), ("padding-right", "${1}")];
        let style = HashMap::from(options);
        let mut rules = HashMap::new();

        let rule = Rule {
            matcher: "Fz",
            allow_param_tovalue: true,
            styles: Style::HashMap(style),
        };

        let rule1 = Rule {
            matcher: "Px",
            allow_param_tovalue: true,
            styles: Style::CallBack(Box::new(|args|{
                format!("padding-x:{}",args[0])
            })),
        };

        rules.insert(rule.matcher, rule);
        rules.insert(rule1.matcher, rule1);

        Rules { rules }
    }
}

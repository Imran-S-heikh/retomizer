use std::collections::HashMap;

use crate::Class;

// struct Object<'a>(HashMap<&'a str, &'a str>);
// struct CallBack(Box<dyn Fn(&Vec<&str>) -> String>);
pub enum Style<'a> {
    Object(HashMap<&'a str, &'a str>),
    CallBack(Box<dyn Fn(&Vec<&str>) -> String>),
}

impl<'a> Style<'a> {
    fn mapped<const T: usize> (array:  [(&'a str,&'a str);T])-> Style<'a>{
        let mut map = HashMap::new();
        for (rule,value) in array {
            map.insert(rule, value);
        }

        Style::Object(map)
    }

    fn callback(cb: &'static dyn Fn(&Vec<&str>)->String)-> Style<'a> {
        Style::CallBack(Box::new(cb))
    }
}

pub struct Rule<'a> {
    matcher: &'a str,
    allow_param_tovalue: bool,
    pub styles: Style<'a>,
}
// const NAMWE: usize = 384;

pub struct Rules<'a> {
    pub rules: HashMap<&'a str, Rule<'a>>,
}

fn handle_styles(this: &Rule, class: Class) {}

fn take_array<const M: usize>(arr:[(&str,&str);M])-> String{
    println!("{:?}",arr);
    String::from("helloworld")
}

impl<'a> Rules<'a> {
    pub fn new() -> Rules<'a> {
        let mut rules = HashMap::new();

        // take_array([("hello","world"),"shaikh"]);
        // take_array(["imran","shaikh"]);
        // take_array(["imran","shaikh","data"]);

        let rule = Rule {
            matcher: "Fz",
            allow_param_tovalue: true,
            styles: Style::mapped([("name","imran")]),
        };

        let rule1 = Rule {
            matcher: "Px",
            allow_param_tovalue: true,
            styles: Style::CallBack(Box::new(|args| format!("padding-x:{}", args[0]))),
        };

        rules.insert(rule.matcher, rule);
        rules.insert(rule1.matcher, rule1);

        Rules { rules }
    }
}

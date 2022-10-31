use std::collections::HashMap;

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
    pub name: &'a str,
    pub matcher: &'a str,
    pub param_tovalue: bool,
    pub arguments: Option<HashMap<&'a str, &'a str>>,
    pub styles: Style<'a>,
}

pub struct Rules<'a> {
    pub rules: HashMap<&'a str, Rule<'a>>,
}

impl<'a> Rules<'a> {
    pub fn mapped() -> HashMap<&'a str, Rule<'a>> {
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
                matcher: "Bgc",
                name: "Background Color",
                param_tovalue: true,
                styles: Style::CallBack(Box::new(|args| {
                    vec![format!("background-color: {}", args[0])]
                })),
                arguments: None,
            },
            Rule {
                matcher: "C",
                name: "Text Color",
                param_tovalue: true,
                styles: Style::mapped([("color", "${0}")]),
                arguments: None,
            },
            Rule {
                matcher: "D",
                name: "Display",
                param_tovalue: false,
                styles: Style::mapped([("display", "${0}")]),
                arguments: Some(HashMap::from([
                    ("n", "none"),
                    ("b", "block"),
                    ("f", "flex"),
                    ("g", "grid"),
                    ("i", "inline"),
                    ("ib", "inline-block"),
                    ("if", "inline-flex"),
                    ("ig", "inline-grid"),
                    ("tb", "table"),
                    ("tbr", "table-row"),
                    ("tbc", "table-cell"),
                    ("li", "list-item"),
                    ("ri", "run-in"),
                    ("cp", "compact"),
                    ("itb", "inline-table"),
                    ("tbcl", "table-column"),
                    ("tbclg", "table-column-group"),
                    ("tbhg", "table-header-group"),
                    ("tbfg", "table-footer-group"),
                    ("tbrg", "table-row-group"),
                ])),
            },
            Rule {
                matcher: "Fz",
                name: "Font Size",
                param_tovalue: true,
                styles: Style::mapped([("font-size", "${0}")]),
                arguments: None,
            },
            Rule {
                matcher: "Px",
                name: "Padding Left and Right",
                param_tovalue: true,
                styles: Style::mapped([("padding-left", "${0}"), ("padding-right", "${0}")]),
                arguments: None,
            },
        ]
    }
}

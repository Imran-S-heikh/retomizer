mod library;

use library::rules::{Rules, Style};
use regex::Regex;
pub struct Retomizer<'a> {
    content: String,
    rules: Rules<'a>,
}

impl<'a> Retomizer<'a> {
    pub fn new(content: String) -> Retomizer<'a> {
        Retomizer {
            content,
            rules: Rules::new(),
        }
    }

    pub fn get_classes(&self) -> Vec<&str> {
        let re = Regex::new(r"[A-Z][a-z]*\([a-zA-Z0-9,]+\)").unwrap();
        let mut result: Vec<&str> = vec![];

        for cap in re.captures_iter(&self.content) {
            if let Some(match_class) = cap.get(0) {
                result.push(match_class.as_str());
            }
        }

        return result;
    }

    fn generate_css(&self, class: Class) -> String {
        let rules = Rules::new().rules;

        if let Some(rule) = rules.get(class.style) {
            let style = match &rule.styles {
                Style::CallBack(callback) => {
                    callback(&class.arguments)
                }
                Style::Object(map) => {
                    String::from("border-radius: 3rem;")
                }
            };

            let selector = format!(r"{}\({}\)",class.style,class.arguments.join(","));

            return format!(r".{selector}{{{style}}}");
        }

        String::from("nothing found")
    }

    pub fn get_css(&self, classes: Vec<&str>) -> String {
        let mut stylesheet = vec![];

        for name in classes {
            if let Some(class) = Class::new(name) {
                let css = Retomizer::generate_css(&self, class);

                stylesheet.push(css)
            }
        }

        stylesheet.join("\n")
    }
}

pub struct Class<'a> {
    name: &'a str,
    style: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> Class<'a> {
    pub fn new(name: &str) -> Option<Class> {
        let regex =
            Regex::new(r"(?P<style>[A-Z][a-z]*)(?:\()(?P<argument>[a-z0-9,]+)(?:\))").unwrap();

        match regex.captures(name) {
            Some(captures) => {
                let style = captures.name("style");
                let argument = captures.name("argument");

                match (style, argument) {
                    (Some(style), Some(argument)) => {
                        return Some(Class {
                            name,
                            style: style.as_str(),
                            arguments: argument.as_str().split(",").collect(),
                        })
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_classes() {
        let content = String::from("Fz(2rem) Fw(5px) D(g) \n Mstart(4px) C(red)");

        let retomizer = Retomizer::new(content);
        let class_names = retomizer.get_classes();

        assert_eq!(
            vec!["Fz(2rem)", "Fw(5px)", "D(g)", "Mstart(4px)", "C(red)"],
            class_names
        );
    }

    #[test]
    fn test_class() {
        let class = Class::new("P(3rem,3rem,10px,34inch)").unwrap();
        assert!(class.name != "");
        assert_eq!(class.arguments, ["3rem", "3rem", "10px", "34inch"]);
        assert_eq!(class.style, "P");
    }
}

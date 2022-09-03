mod library;

use library::rules::{Rules, Style};
use regex::{Match, Regex};
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
        let re = Regex::new(r"[A-Z][a-z]*\([a-zA-Z0-9,%]+\)(?:!)?(?::[a,c,f,h])?(?:::[a,b,fl,fli,ph])?(?:--[a-z]+)?").unwrap();
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
                Style::CallBack(callback) => callback(&class.arguments),
                Style::Object(map) => {
                    let mut styles = vec![];

                    for (style, value) in map {
                        let mut value = value.to_string();
                        for (i, arg) in class.arguments.iter().enumerate() {
                            value = value.replace(format!("${{{i}}}").as_str(), arg);
                        }
                        // println!("{value}--------------");
                        styles.push(format!("{style}:{value}"));
                    }

                    // styles.join(";")
                    styles
                }
            };

            let css_class = class.to_string(style);

            return css_class;
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
    important: bool,
    psudo_class: Option<Match<'a>>,
}

impl<'a> Class<'a> {
    pub fn new(name: &str) -> Option<Class> {
        let regex = Regex::new( r"(?P<context>[a-zA-Z]+(?P<context_psudo_class>:(a|f|c|h))?(?P<combinator>(_|>|~|\+)))?(?P<style>[A-Z][a-z]*)(?:\()(?P<arguments>[a-z0-9,]+)(?:\))(?P<important>!)?(?P<psudo_class>:(a|f|c|h))?(?P<psudo_element>::(a|bd|b|c|fsb|fli|fl|m|ph|s))?(?P<breakpoint>--[a-z0-9]+)?").unwrap();

        match regex.captures(name) {
            Some(captures) => {
                let style = captures.name("style");
                let arguments = captures.name("arguments");
                let important = captures.name("important").is_some();
                let psudo_class = captures.name("psudo_class");

                match (style, arguments) {
                    (Some(style), Some(arguments)) => {
                        return Some(Class {
                            name,
                            important,
                            psudo_class,
                            style: style.as_str(),
                            arguments: arguments.as_str().split(",").collect(),
                        })
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn format_properties(&self, properties: &Vec<String>) -> String {
        properties
            .into_iter()
            .map(|property| {
                format!(
                    "{property}{important}",
                    important = if self.important { " !important" } else { "" }
                )
            })
            .collect::<Vec<String>>()
            .join(";")
    }

    pub fn to_string(&self, properties: Vec<String>) -> String {
        let selector = format!(
            r"{context}{style}\({arguments}\){important}{psudo_class}{psudo_element}{breakpoint}",
            context = "",
            style = self.style,
            arguments = self.arguments.join(","),
            important = if self.important { r"\!" } else { "" },
            psudo_class = "",
            psudo_element = "",
            breakpoint = ""
        );

        let style = format!(
            r"{{{properties}}}",
            properties = Class::format_properties(&self, &properties)
        );
        return format!(".{selector}{style}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_classes() {
        let content =
            String::from("Fz(2rem) Fw(5px) D(g) \n Mstart(4px)--sm C(red):h::b--sm flex flex-1");

        let retomizer = Retomizer::new(content);
        let class_names = retomizer.get_classes();

        assert_eq!(
            vec![
                "Fz(2rem)",
                "Fw(5px)",
                "D(g)",
                "Mstart(4px)--sm",
                "C(red):h::b--sm"
            ],
            class_names
        );
    }

    #[test]
    fn test_class() {
        let class = Class::new("P(3rem,3rem,10px,34inch)");
        let class = match class {
            Some(class) => class,
            None => panic!("Failed To Generate class"),
        };
        assert!(class.name != "");
        assert_eq!(class.arguments, ["3rem", "3rem", "10px", "34inch"]);
        assert_eq!(class.style, "P");
    }

    #[test]
    fn test_important() {
        // important should be false here
        {
            let class = Class::new("P(3rem,3rem,10px,34inch)").unwrap();
            assert!(!class.important);
        }

        // important should be true
        {
            let class = Class::new("P(3rem,3rem,10px,34inch)!").unwrap();
            assert!(class.important);
        }

        // important should be false because, its used in wrong place
        {
            let class = Class::new("P(3rem,3rem,10px,34inch):h!").unwrap();
            assert!(!class.important);
        }
    }

    #[test]
    fn test_hover() {
        {
            let class = Class::new("P(2rem):h");
        }
    }
}

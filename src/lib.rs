mod library;
use std::{collections::HashMap, path::Path};

use library::rules::{Rule, Rules, Style};
use regex::{Captures, Match, Regex};

pub use crate::library::config::Config;

pub struct MediaQuery<'a> {
    query: &'a str,
    css: String,
    pixel: i32,
    breakpoint: String,
}

impl<'a> MediaQuery<'a> {
    pub fn new(query: &'a str, css: String, breakpoint: String) -> Self {
        let regex = Regex::new("[0-9]+").unwrap();
        let message = "MediaQuery Should Have a Breakpoint";
        let pixel = regex
            .captures(query)
            .expect(message)
            .get(0)
            .expect(message)
            .as_str();
        let pixel: i32 = pixel.parse().unwrap();

        return Self {
            query,
            css,
            pixel,
            breakpoint,
        };
    }
}

pub struct Retomizer<'a> {
    rules: HashMap<&'a str, Rule<'a>>,
    config: &'a Config,
    stylesheet: HashMap<String, String>,
    mediaquery: HashMap<String, MediaQuery<'a>>,
}

impl<'a> Retomizer<'a> {
    pub fn new(config: &'a Config) -> Retomizer<'a> {
        Retomizer {
            config,
            rules: Rules::mapped(),
            stylesheet: HashMap::new(),
            mediaquery: HashMap::new(),
        }
    }

    pub fn get_classes(&self, content: String) -> Vec<String> {
        let re = Regex::new(r"[A-Z][a-z]*\([a-zA-Z0-9,%]+\)(?:!)?(?::(a|c|f|h))?(?:::(a|bd|b|c|fsb|fli|fl|m|ph|s))?(?:--[a-z]+)?").unwrap();
        let mut result: Vec<String> = vec![];

        for cap in re.captures_iter(&content) {
            if let Some(match_class) = cap.get(0) {
                let class = match_class.as_str().to_string();

                if !self.stylesheet.contains_key(&class) {
                    result.push(class);
                }
            }
        }

        return result;
    }

    fn generate_css(&self, class: &Class) -> Option<String> {
        let rules = &self.rules;

        if let Some(rule) = rules.get(class.style) {
            let style = match &rule.styles {
                Style::CallBack(callback) => callback(&class.arguments),
                Style::Object(map) => {
                    let mut styles = vec![];

                    for (style, value) in map {
                        let mut value = value.to_string();

                        // Set The Arg with placeholder
                        for (i, arg) in class.arguments.iter().enumerate() {
                            let arg = if rule.param_tovalue {
                                *arg
                            } else {
                                let msg =
                                    format!("Bad Rules, No Arguments Defined for {}", rule.name);
                                let valid_args = rule.arguments.as_ref().expect(&msg);
                                let msg =
                                    format!("🚫 {arg} is not a valid argument for {}", rule.name);
                                let arg = *valid_args.get(arg).expect(&msg);

                                arg
                            };
                            value = value.replace(format!("${{{i}}}").as_str(), arg);
                        }
                        styles.push(format!("{style}:{value}"));
                    }

                    styles
                }
            };

            let css_class = class.to_string(style, &self.config);

            return Some(css_class);
        }

        None
    }

    pub fn get_css(&self) -> String {
        let stylesheet = &self.stylesheet;
        let mut stylesheet: Vec<String> = stylesheet.clone().into_values().collect();
        stylesheet.sort();
        let styles = stylesheet.join("\n");

        let mut cache: HashMap<&String, Vec<&String>> = HashMap::new();
        let mut breakpoint_map:  HashMap<&String,i32> = HashMap::new();

        for (_, mediaquery) in self.mediaquery.iter() {
            let key = &mediaquery.breakpoint;
            let css = &mediaquery.css;

            if cache.contains_key(key) {
                let arr = cache.get_mut(key).unwrap();
                arr.push(css);
            } else {
                cache.insert(key, vec![css]);
            }

            breakpoint_map.insert(key, mediaquery.pixel);
        }

        let mut points: Vec<&String> = breakpoint_map.clone().into_keys().collect();

        points.sort_by(|a,b|{
            let val_a = breakpoint_map.get(a).unwrap();
            let val_b = breakpoint_map.get(b).unwrap();

            val_a.partial_cmp(val_b).unwrap()
        });

        let mut styles_media: Vec<String> = Vec::new();

        for key in points {
            let styles = cache.get_mut(key).unwrap();
            let query = self.config.breakpoints.get(key).unwrap();
            styles.sort();
            let styles: Vec<String> = styles.iter().map(|e|e.to_string()).collect();

            let styles = styles.join("\n");

            styles_media.push(format!("{query}{{\n{styles}\n}}"));
        }

        let styles_media = styles_media.join("\n");

        format!("{styles}\n{styles_media}")
    }

    pub fn push_content(&mut self, content: String) {
        let classes = self.get_classes(content);

        for name in classes {
            if let Some(class) = Class::new(&name) {
                let key = class.get_selector();
                let css = self.generate_css(&class);
                let breakpoint = Class::get_match(class.breakpoint);
                let mediaquery = self.config.breakpoints.get(&breakpoint);

                match (mediaquery, class.breakpoint, css) {
                    (Some(mediaquery), Some(breakpoint), Some(css)) => {
                        let mediaquery =
                            MediaQuery::new(mediaquery, css, breakpoint.as_str().to_owned());
                        self.mediaquery.insert(key, mediaquery);
                    }
                    (None, None, Some(css)) => {
                        self.stylesheet.insert(key, css);
                    }
                    _ => (),
                }
            }
        }
    }

}

pub struct Psudo<'a> {
    selector: &'a str,
    value: &'a str,
}

impl<'a> Psudo<'a> {
    pub fn new(psudo: Option<Match>) -> Psudo {
        let psudo = Class::get_match(psudo);
        let (selector, value) = match psudo.as_str() {
            ":a" => (":a", ":active"),
            ":c" => (":c", ":checked"),
            ":f" => (":f", ":focus"),
            ":h" => (":h", ":hover"),
            "::a" => ("::a", "::after"),
            "::b" => ("::b", "::before"),
            "::bd" => ("::bd", "::backdrop"),
            "::c" => ("::c", "::cue"),
            "::fsb" => ("::fsb", "::file-selector-button"),
            "::fl" => ("::fl", "::first-letter"),
            "::fli" => ("::fli", "::first-line"),
            "::m" => ("::m", "::marker"),
            "::ph" => ("::ph", "::placeholder"),
            "::s" => ("::s", "::selection"),
            _ => ("", ""),
        };

        Psudo { selector, value }
    }
}
pub struct Class<'a> {
    name: &'a str,
    style: &'a str,
    arguments: Vec<&'a str>,
    important: bool,
    psudo_class: Option<Match<'a>>,
    psudo_element: Option<Match<'a>>,
    breakpoint: Option<Match<'a>>,
}

impl<'a> Class<'a> {
    pub fn new(name: &str) -> Option<Class> {
        let regex = Regex::new( r"(?P<context>[a-zA-Z]+(?P<context_psudo_class>:(a|f|c|h))?(?P<combinator>(_|>|~|\+)))?(?P<style>[A-Z][a-z]*)(?:\()(?P<arguments>[a-z0-9,]+)(?:\))(?P<important>!)?(?P<psudo_class>:(a|f|c|h))?(?P<psudo_element>::(a|bd|b|c|fsb|fli|fl|m|ph|s))?(--(?P<breakpoint>[a-z0-9]+))?").unwrap();

        match regex.captures(name) {
            Some(captures) => {
                let style = captures.name("style");
                let arguments = captures.name("arguments");

                match (style, arguments) {
                    (Some(style), Some(arguments)) => {
                        return Some(Class {
                            name,
                            important: captures.name("important").is_some(),
                            psudo_class: captures.name("psudo_class"),
                            psudo_element: captures.name("psudo_element"),
                            breakpoint: captures.name("breakpoint"),
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

    fn get_match(text: Option<Match>) -> String {
        match text {
            Some(value) => value.as_str().to_string(),
            None => String::from(""),
        }
    }

    pub fn get_selector(&self) -> String {
        let psudo_class = Psudo::new(self.psudo_class);
        let psudo_element = Psudo::new(self.psudo_element);
        let breakpoint = Class::get_match(self.breakpoint);

        format!(
            r"{context}{style}({arguments}){important}{psudo_class}{psudo_element}{breakpoint}",
            context = "",
            style = self.style,
            arguments = self.arguments.join(","),
            important = if self.important { "!" } else { "" },
            psudo_class = psudo_class.selector,
            psudo_element = psudo_element.selector,
            breakpoint = if breakpoint == "" {
                String::from("")
            } else {
                format!("--{breakpoint}")
            }
        )
    }

    pub fn to_string(&self, properties: Vec<String>, config: &Config) -> String {
        let psudo_class = Psudo::new(self.psudo_class);
        let psudo_element = Psudo::new(self.psudo_element);
        // let breakpoint = Class::get_match(self.breakpoint);
        // let mediaquery = config.breakpoints.get(&breakpoint);
        let selector = self.get_selector();

        let regex = Regex::new(r"[!():]").unwrap();
        let selector = regex.replace_all(&selector, |capture: &Captures| {
            let escaped = match capture.get(0) {
                Some(value) => format!(r"\{}", value.as_str()),
                None => String::from(""),
            };

            escaped
        });

        let psudo = format!(
            "{class}{element}",
            class = psudo_class.value,
            element = psudo_element.value
        );

        let style = format!(
            r"{{{properties}}}",
            properties = Class::format_properties(&self, &properties)
        );

        let class = format!(".{selector}{psudo}{style}");
        // if let Some(media) = mediaquery {
        //     class = format!("{media}{{{class}}}")
        // }

        return class;
    }
}

#[cfg(test)]
#[deny(soft_unstable)]
mod tests {

    use super::*;

    #[test]
    fn test_get_classes() {
        let content =
            String::from("Fz(2rem) Fw(5px) D(g) \n Mstart(4px)--sm C(red):h::b--sm flex flex-1");
        let config = Config::default();
        let retomizer = Retomizer::new(&config);
        let class_names = Retomizer::get_classes(&retomizer, content);
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
    fn test_psudo_class() {
        {
            let class = Class::new("P(2rem):h");
            let psudo = class.unwrap().psudo_class.unwrap();

            assert_eq!(":h", psudo.as_str());
        }

        {
            let class = Class::new("P(2rem)::b:h");
            let psudo = class.unwrap().psudo_class;

            if let Some(_) = psudo {
                panic!("Should Not Get Any Psudo Class")
            }
        }

        {
            let class = Class::new("P(2rem):f::b--sm");
            let psudo = class.unwrap().psudo_class;

            if let None = psudo {
                panic!("Should Get A Psudo Class")
            }
        }
    }

    #[test]
    fn test_psudo_element() {
        {
            let class = Class::new("P(2rem):h::b");
            let psudo = class.unwrap().psudo_element.unwrap();

            assert_eq!("::b", psudo.as_str());
        }
        {
            let class = Class::new("P(2rem):h--sm::b");
            let psudo = class.unwrap().psudo_element;

            if let Some(_) = psudo {
                panic!("Should Not Get Any Psudo Class")
            }
        }

        {
            let class = Class::new("P(2rem):f::b--sm");
            let psudo = class.unwrap().psudo_element;

            if let None = psudo {
                panic!("Should Get A Psudo Class")
            }
        }
    }
}

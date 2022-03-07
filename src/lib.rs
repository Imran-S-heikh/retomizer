use regex::Regex;
struct Retomizer {
    content: String,
}

impl Retomizer {
    fn new(content: String) -> Retomizer {
        Retomizer { content }
    }

    fn get_classes(&self) -> Vec<&str> {
        let re = Regex::new(r"[A-Z][a-z]*\([a-zA-Z0-9,]+\)").unwrap();
        let mut result: Vec<&str> = vec![];

        for cap in re.captures_iter(&self.content) {
            if let Some(match_class) = cap.get(0) {
                result.push(match_class.as_str());
            }
        }

        return result;
    }
}

struct Class<'a> {
    name: &'a str,
    style: &'a str,
    arguments: Vec<&'a str>
}

impl<'a> Class<'a> {
    fn new(name: &str)-> Option<Class> {
        let regex = Regex::new(r"(?P<style>[A-Z][a-z]*)(?:\()(?P<argument>[a-z0-9,]+)(?:\))").unwrap();

        match regex.captures(name) {
            Some(captures) => {
                let style = captures.name("style");
                let argument = captures.name("argument");

                match (style,argument) {
                    (Some(style),Some(argument))=> {
                        return Some(Class {
                            name,
                            style: style.as_str(),
                            arguments: argument.as_str().split(",").collect(),
                        })
                    }
                    _ => None
                }
            },
            None => None
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

        assert_eq!(vec!["Fz(2rem)", "Fw(5px)", "D(g)","Mstart(4px)","C(red)"], class_names);
    }

    #[test]
    fn test_class () {

        let class = Class::new("P(3rem,3rem,10px,34inch)").unwrap();
        assert!(class.name != "");
        assert_eq!(class.arguments,["3rem","3rem","10px","34inch"]);
        assert_eq!(class.style,"P");
    }
}

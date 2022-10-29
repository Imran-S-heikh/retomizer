use std::collections::HashMap;

use regex::Regex;

pub fn argmap(args: Vec<String>) -> HashMap<String,Vec<String>>{
    let mut map = HashMap::new();
    let regex = Regex::new("(-[a-zA-Z])|(--[A-Za-z]+)").unwrap();
    let hypene = Regex::new("-+").unwrap();

    for (index,arg) in args.iter().enumerate() {
        if regex.is_match(&arg) {
            let key = hypene.replace(&arg, "").to_string();
            let is_contains = map.contains_key(&key);
            if !is_contains {
                map.insert(key.clone(), vec![]);
            }
            match args.get(index+1) {
                Some(next)=>{
                    if !regex.is_match(next) {
                        let arr = &mut *map.get_mut(&key).unwrap();
                        arr.push(next.clone());
                    }
                },
                None=>()
            }
        }
    } 

    return map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argmap() {
        let args = vec![
            String::from("--config"),
            String::from("./lib.rs"),
            String::from("--watch"),
        ];
        let mapped = argmap(args);

        match mapped.get("config") {
            Some(value)=>{
                assert_eq!(value[0],"./lib.rs");
            },
            None=> panic!("Expected To have config")
        }

        assert!(mapped.contains_key("watch"));

    }
}

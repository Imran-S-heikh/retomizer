struct Retomizer {
    content: String
}

impl Retomizer {
    fn new(content: String)-> Retomizer {
       Retomizer{
           content
       } 
    }

    fn get_classes(&self)-> Vec<&str> {
       vec![] 
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_get_classes() {
        let content = String::from("Fz(2rem) Fw(5px) D(g)");

        let retomizer = Retomizer::new(content);
        let class_names = retomizer.get_classes();

        assert_eq!(vec!["Fz(2rem)","Fw(5px)","D(g)"],class_names);
    }
}
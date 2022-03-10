mod tests {
    use std::fs;

    use retomizer::Retomizer;


    #[test]
    fn test_module() {
        let content = fs::read_to_string("./tests/files/index.html").unwrap();

        let retomizer = Retomizer::new(content);

        let classes = retomizer.get_classes();
        let css = retomizer.get_css(classes);

        println!("{css}");
        assert!(false);
    }

}
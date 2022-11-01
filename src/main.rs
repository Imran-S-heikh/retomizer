use std::{
    env,
    fs::{self, read_to_string, write},
    path::Path,
    sync::mpsc::channel,
};

use argmap::argmap;
use glob::glob;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use retomizer::{Config, Retomizer};

fn main() -> Result<(), ()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
    let args: Vec<String> = env::args().collect();
    let mapped = argmap(args);
    let build = !mapped.contains_key("watch");

    let pwd = env::current_dir().unwrap();
    let path = mapped.get("config").unwrap().get(0).unwrap();
    let config_path = pwd.join(path).canonicalize().unwrap();
    let config = Config::load(&config_path);
    let mut base_path = config_path.clone();
    base_path.pop();

    let mut retomizer = Retomizer::new(&config);
    let output = base_path
        .join(&config.output)
        .canonicalize()
        .expect("Failed To Get Output File");

    for path in &config.content {
        let base_path = base_path.clone();
        // base_path.pop();

        let abs_path = format!("{}", base_path.join(&path).display());
        let paths = glob(&abs_path).unwrap();

        for path in paths {
            match path {
                Ok(path) => {
                    let path = path.canonicalize().unwrap();

                    let content = fs::read_to_string(&path).unwrap();

                    retomizer.push_content(content);

                    if !build {
                        watcher
                            .watch(path.as_path(), RecursiveMode::NonRecursive)
                            .unwrap();
                    }
                    println!(
                        "🚀 {}: {}",
                        if build { "Checking" } else { "Watching" },
                        path.display()
                    )
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    let css = retomizer.get_css();
    let message = format!("Failed To Write at {}", output.display());
    write(&output, css).expect(&message);

    if !build {
        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() {
                        let path = Path::new(&event.paths[0]);
                        let content = read_to_string(path).unwrap();
                        retomizer.push_content(content);
                        let css = retomizer.get_css();

                        write(&output, css).unwrap();
                    }
                }
                Err(e) => println!("watcher error: {:?}", e),
            }
        }
    }

    Ok(())
}

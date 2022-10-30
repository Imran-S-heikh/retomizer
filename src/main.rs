use std::{env, sync::mpsc::channel, fs};

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
    let mut retomizer = Retomizer::new(&config);

    for path in &config.content {
        let mut base_path = config_path.clone();
        base_path.pop();

        let abs_path = format!("{}", base_path.join(&path).display());
        let paths = glob(&abs_path).unwrap();

        for path in paths {
            match path {
                Ok(path) => {
                    let path = path.canonicalize().unwrap();

                    let content = fs::read_to_string(&path).unwrap();

                    retomizer.push_content(content);

                    // watcher
                    //     .watch(path.as_path(), RecursiveMode::NonRecursive)
                    //     .unwrap();
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

    if !build {
        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_access() {
                        let s = event.source();

                        match s {
                            Some(val) => println!("{val}"),
                            None => (),
                        }

                        println!("{:?}", event)
                    }
                }
                Err(e) => println!("watcher error: {:?}", e),
            }
        }
    }else {
        let css = retomizer.get_css();
        println!("{css}");
    }

    Ok(())
}

use std::fs;


pub fn list_of_files() -> Vec<String> {
    let paths = fs::read_dir("./").unwrap();
    let mut vec = Vec::new();

    for path in paths {
        let s = String::from(path.unwrap().path().to_str().unwrap());
        vec.push(s)
    }

    vec
}

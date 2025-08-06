fn main() {
    let path = {
        let mut args = std::env::args();
        args.nth(1).unwrap_or(".".to_string())
    };
    let metadata = std::fs::metadata(&path).expect("Given path is invalid");

    if metadata.is_dir() {
        let dir_content = std::fs::read_dir(path).expect("Shouldnt crash, but whatever !");
        for entry in dir_content {
            println!("{}", entry.unwrap().file_name().to_str().unwrap());
        }
    } else if metadata.is_file() {
        let path = std::path::PathBuf::from(path);
        println!("{}", path.file_name().unwrap().to_str().unwrap());
    }
}

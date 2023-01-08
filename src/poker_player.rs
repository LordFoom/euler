use std::io::BufReader;

fn read_poker_file() {
    //read in the poker file's contects from the file system
    let pf_contents =
        BufReader::new(std::fs::File::open("poker_054.txt").expect("Cannot find file"));
}

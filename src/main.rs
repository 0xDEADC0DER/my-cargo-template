fn main() {
    let author_name = "{{authors}}".split(' ').next().unwrap_or(""); 

    println!("Welcome to the Project, {author_name}!");
}

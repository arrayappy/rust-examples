fn main() {
    let users_first_names = ["Naidu", "John", "Rakesh", "Priya"];
    let users_last_names = ["Sunny", "Frank", "Kumar", "Mishra"];

    if users_first_names.len() == users_last_names.len() {
        for (i, name) in users_first_names.iter().enumerate() {
            println!("{} {}", name, users_last_names[i]);
        }
    }
}
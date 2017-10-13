pub fn parse(buf: &str) -> Vec<String> {

    let mut token: String = String::from("");

    let mut v: Vec<String> = Vec::new();
    
    for c in buf.chars() {
        if c == ' ' {
            v.push(token.clone());
            token = String::from("");
        } else if c == '\n' {
            v.push(token.clone());
            token = String::from("");
        } else {
            token.push(c);
        }
    }

    println!("{}", v[0]); 
    
    v
}

// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // ???
    let mut start_endl:usize  = input.len();
    let mut startl:usize  = 0;

    let endl:usize  = input.len();

    let mut at_end = false;
    let mut at_start = true;

    let fake = String::from(input);
    let mut begining_white:&str;
    let bytes = input.as_bytes();
    for (x,&i)in bytes.iter().enumerate(){
        if i != b' '{
            if at_start{
                startl = x;
            }
            at_start = false;

            at_end=false;
        }

        if i == b' '{
            if!at_end{
                start_endl = x
            }
            at_end=true; 
        }

    }

    if at_end == false {
        start_endl = endl
    }

    let fartman = &fake[startl..start_endl];
    fartman.to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    // ???

    let x = String::from(input.to_owned() + " world!");
    x
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // ???
    let mut s = String::from(input);
    s.replace_range(input.find("cars").unwrap_or(s.len())..input.find("cars").unwrap_or(s.len())+4,"balloons");
    
    String::from(s)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}

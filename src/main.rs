use without_comments::IntoWithoutComments;

fn main() {
    let text = "S/*he */be/*lie*/ve//d";
    let truth = text.chars().without_comments().collect::<String>();
    println!("{}", truth);
}
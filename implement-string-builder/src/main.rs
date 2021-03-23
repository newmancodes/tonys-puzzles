mod string_builder;

use string_builder::StringBuilder;

fn main() {
    let mut sb = StringBuilder::new();

    sb.append("Hello");
    sb.append(", ");
    sb.append("world!");

    println!("{}", String::from(sb));
}

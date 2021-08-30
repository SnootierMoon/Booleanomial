use booleanomial::Booleanomial;

fn main() {
    let a = Booleanomial::<8>::new(0);
    let b = Booleanomial::<8>::new(1);
    let c = Booleanomial::<8>::new(2);
    println!("~a\n    = ({})", a.not());
    println!("a & b\n    = ({})", a.and(&b));
    println!("b | a\n    = ({})", a.or(&b));
    println!("a ^ b\n    = ({})", a.xor(&b));
    println!("a & (b | c)\n    = ({})", b.or(&c).and(&a));
    println!("S = a ^ (b ^ c)\n    = ({})", a.xor(&b).xor(&c));
    println!(
        "cout = (a & b) | (c & (a ^ b))\n    = ({})",
        a.and(&b).or(&c.and(&a.xor(&b)))
    )
}

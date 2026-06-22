fn main() {
    for i in 0..500u64 {
        let x = lds_gen::vdc(i, 2);
        let y = lds_gen::vdc(i, 3);
        println!("{:.16},{:.16}", x, y);
    }
}

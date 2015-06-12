extern crate fixedpt;


fn main() {
    let mut fp = fixedpt::Fixed32::new(3);
    let mut fq = fixedpt::Fixed32::new(3);
    fp.setf(7.0);

    fq.setf(2.5);

    let fr = fq << 2;
    

    println!("{}", fr.geti());

    let fr = fr >> 2;
    println!("{}", fr.geti());
    
}

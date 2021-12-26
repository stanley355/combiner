fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    println!("{:?} {:?}", v1_iter.next(), Some(&1));
    println!("{:?} {:?}",v1_iter.next(), Some(&2));
    println!("{:?} {:?}",v1_iter.next(), Some(&3));
}

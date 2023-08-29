fn main(){
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();


    for val in v1_iter{
        println!("Got: {}", val)
    }

    //-------------------------------------

    // Double check to see if it works
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));

    //-------------------------------------

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

}
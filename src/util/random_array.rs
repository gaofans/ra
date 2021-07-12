use rand::{thread_rng, Rng};

pub fn random_array(size:usize,max:usize) -> Vec<usize> {
    let mut arr = Vec::with_capacity(size);
    let mut rng = thread_rng();
    for _ in 0..size {
        arr.push(rng.gen_range(0..max));
    }
    arr
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let vec = random_array(100, 1000);
        println!("{:?}",vec);
        assert_eq!(vec.len(),100);
    }
}
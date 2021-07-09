pub fn insertion_sort<T:Ord>(arr:&mut[T]) -> &mut[T]{
    for i in 0..arr.len() {
        if i == 0{
            continue
        }
        for j in (0..i+1).rev() {
            if j==0 {
                break;
            }
            if arr[j] < arr[j-1]{
                arr.swap(j,j-1)
            }else{
                break;
            }
        }
    }
    arr
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn sort() {
        let mut arr = vec!["23","3","123","43","22","98","12","1","68"];
        let sort = insertion_sort(&mut arr);
        println!("{:?}",sort);
        for i in 0..arr.len() -1 {
            assert!(arr[i] <= arr[i+1])
        }
    }
}
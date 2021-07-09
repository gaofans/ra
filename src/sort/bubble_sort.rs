///冒泡排序算法的原理如下： [1]
/// 比较相邻的元素。如果第一个比第二个大，就交换他们两个。
/// 对每一对相邻元素做同样的工作，从开始第一对到结尾的最后一对。在这一点，最后的元素应该会是最大的数。
/// 针对所有的元素重复以上的步骤，除了最后一个。
/// 持续每次对越来越少的元素重复上面的步骤，直到没有任何一对数字需要比较。
pub fn bubble_sort<T:Ord>(arr:&mut [T]) -> &mut [T]{
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j+1] {
               arr.swap(j,j+1);
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
        let mut arr = vec![23,3,123,43,22,98,12,1,68];
        let sort = bubble_sort(&mut arr);
        println!("{:?}",sort);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i+1])
        }
    }
}
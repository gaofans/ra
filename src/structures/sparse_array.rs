
pub fn parse_array<T:Copy>(data:&[Vec<Option<T>>]) -> Result<Vec<ArrInfo<T>>, Error>{
    if data.len() == 0 {
        return Err(Error(String::from("数组高度不得为0")))
    }
    let mut res = Vec::with_capacity(data.len());
    let width = data[0].len();
    for y in 0..data.len() {
        if data[y].len() != width {
            return Err(Error(String::from("数组宽度必须一致")))
        }
        for x in 0..data[y].len() {
            if let Some(value) = data[y][x] {
                res.push(ArrInfo::new(x,y,value));
            }
        }
    }
    Ok(res)
}

pub fn reset(res:&Vec<ArrInfo<T>>) -> Box<[Vec<Option<T>>]>{

}
#[derive(Debug)]
pub struct ArrInfo<T:Copy>{
    x:usize,
    y:usize,
    value:T
}
impl <T:Copy> ArrInfo<T>{
    fn new(x:usize,y:usize,value:T) -> ArrInfo<T>{
        ArrInfo{x,y,value}
    }
}
#[derive(Debug)]
pub struct Error(String);


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let row1 = vec![None,None,None,None,Some(12),None,None];
        let row2 = vec![None,None,Some(14),None,Some(18),None,None];
        let row3 = vec![None,Some(17),None,Some(56),None,Some(4),None];
        let data = [row1,row2,row3];
        println!("{:?}",parse_array(&data).unwrap());
    }
}
pub fn parse_array<T:Copy>(data:&[Vec<Option<T>>]) -> Result<Vec<Metadata<T>>, Error>{
    if data.len() == 0 {
        return Err(Error(String::from("数组高度不得为0")))
    }
    let mut size = 0;
    for y in data {
        for x in y {
            size += 1;
        }
    }
    let mut res = Vec::with_capacity(size+1);
    res.push(Metadata::Head {x:data.len(),y:data.get(0).unwrap().len(),size});
    let width = data[0].len();
    for y in 0..data.len() {
        if data[y].len() != width {
            return Err(Error(String::from("数组宽度必须一致")))
        }
        for x in 0..data[y].len() {
            if let Some(value) = data[y][x] {
                res.push(Metadata::Info(ArrInfo::new(x,y,value)));
            }
        }
    }
    Ok(res)
}

pub fn reset<T:Copy>(res:&Vec<Metadata<T>>) -> Result<Vec<Vec<Option<T>>>,Error>{
    if let Metadata::Head{x,y,size} = res.get(0).unwrap(){
        let mut data:Vec<Vec<Option<T>>>= vec![vec![Option::None;*y];*x];
        for i in 1..res.len() {
            match res.get(i).unwrap() {
                _ => (),
                Metadata::Info(info) => {
                    let mut node = data.get_mut(info.y).unwrap().get_mut(info.x).unwrap();
                    node = &mut Some(info.value);
                }
            }
        }
        return Ok(data);
    }
    Err(Error(String::from("数据不正确")))
}

#[derive(Debug)]
pub enum Metadata<T:Copy>{
    Head{
        x:usize,
        y:usize,
        size:usize
    },
    Info(ArrInfo<T>)
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
        let res = parse_array(&data).unwrap();
        println!("{:?}",parse_array(&data).unwrap());
        println!("{:?}",reset(&res).unwrap());
    }
}
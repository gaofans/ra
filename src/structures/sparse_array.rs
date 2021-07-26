use std::fmt::Debug;
use serde::{Serialize,Deserialize};

pub fn parse_array<T:Copy+Serialize>(data:&[Vec<Option<T>>]) -> Result<Vec<Metadata<T>>, Error>{
    if data.len() == 0 {
        return Err(Error(String::from("数组高度不得为0")))
    }
    let mut size = 0;
    for y in data {
        for _ in y {
            size += 1;
        }
    }
    let mut res = Vec::with_capacity(size+1);
    res.push(Metadata::Head {y:data.len(),x:data.get(0).unwrap().len(),size});
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

pub fn reset<T:Copy+Serialize>(res:&Vec<Metadata<T>>) -> Result<Vec<Vec<Option<T>>>,Error>{
    if let Metadata::Head{x,y,size:_} = res.get(0).unwrap(){
        let mut data:Vec<Vec<Option<T>>> = vec![vec![Option::None;*x];*y];
        for i in 1..res.len() {
            match res.get(i).unwrap() {
                Metadata::Info(info) => {
                    data[info.y][info.x] = Some(info.value);
                },
                _ => ()
            }
        }
        return Ok(data);
    }
    Err(Error(String::from("数据不正确")))
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Metadata<T:Copy+Serialize>{
    Head{
        x:usize,
        y:usize,
        size:usize
    },
    Info(ArrInfo<T>)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ArrInfo<T:Copy+Serialize>{
    x:usize,
    y:usize,
    value:T
}
impl <T:Copy+Serialize> ArrInfo<T>{
    fn new(x:usize,y:usize,value:T) -> ArrInfo<T>{
        ArrInfo{x,y,value}
    }
}
#[derive(Debug)]
pub struct Error(String);
use std::fs::File;
use serde::de::DeserializeOwned;

pub fn serialize<T:Copy+Serialize>(file:File,res:&Vec<Metadata<T>>) {
    serde_json::to_writer(file,res).unwrap();
}

pub fn deserialize<T: DeserializeOwned>(file:File) -> T{
    return serde_json::from_reader(file).unwrap();
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        let row1 = vec![None,None,None,None,Some(12),None,None];
        let row2 = vec![None,None,Some(14),None,Some(18),None,None];
        let row3 = vec![None,Some(17),None,Some(56),None,Some(4),None];
        let data = [row1,row2,row3];
        //压缩
        let res = parse_array(&data).unwrap();
        //序列化到文件
        serialize(File::create("test.txt").unwrap(),&res);
        //反序列化
        let aaa:Vec<Metadata<i32>> = deserialize(File::open("test.txt").unwrap());
        //解压
        println!("{:?}",reset(&aaa).unwrap());
    }
}
extern crate nom;
use nom::{
    branch::alt,
    IResult,
    character::is_alphanumeric,
    character::complete::alpha1,
    bytes::complete::{tag, take_while_m_n, take_while},
    combinator::map,
    sequence::{tuple, preceded},
    character::complete::{digit1, multispace0}};

#[derive(Debug)]
enum op {
    plus,
    minus,
    divide,
    times,
    modulo,
    unknown,
}

#[derive(Debug)]
enum types{
    integer32,
    boolean,
}

#[derive(Debug)]
enum variable{
    name,
    value,
}

#[derive(Debug)]
enum list {
    Cons(Box<list>, op, Box<list>),
    VarBox(Box<list>, op, Box<list>),
    Num(i32),
    Var(String),
}

fn parser(input:&str) -> IResult<&str, &str> {
    digit1(input)
}

fn parserTest(input:&str) -> IResult<&str, &str> {
    preceded(
        multispace0,
        alt((
            digit1,            
            take_while(char::is_alphanumeric),
    )),
    )(input)
}

fn parseInput(input: &str) -> (&str, &str){
    let parsedInput = parserTest(input);
    match parsedInput{
        Ok(k) => k,
        Err(e) => ("i am", "error"),
    }

}

fn BoxInput(input: &str) ->Box<list>{
     match parserTest(input){
        Ok(k) => {
            let isLetter: IResult<&str,&str> = alpha1(k.1);
            match isLetter{
                Ok(r) => {
                    println!("{:?}", r.0);
                    let theSign = getSign(k.0);
                    let new_string = r.1.to_string();
                    if(r.0 == ""){
                        return Box::new(list::Var(new_string))
                    }
                    let VarTreelist = list::VarBox(Box::new(list::Var(new_string)), theSign.1, BoxInput(&k.0[1..]));
                    return Box::new(VarTreelist);
                }Err(t) => (),
            }
            }Err(e) => ()
        ,
    }
    let (signNum, numStr) = parseInput(input);
    let number = numStr.parse::<i32>().unwrap();
    let sign = getSign((signNum));
//    println!("signNum : {:?}", sign);
    if(signNum == ""){
        return Box::new(list::Num(number))
    };
    let treeList = list::Cons(Box::new(list::Num(number)), sign.1, BoxInput(&signNum[1..]));
    return Box::new(treeList);
}



fn getSign(input: &str) ->(&str, op) {
    let sign: IResult<&str, op> = preceded(
        multispace0,
        alt((
            map(tag("+") ,  |_| op::plus), 
            map(tag("-") ,  |_| op::minus),
            map(tag("*") ,  |_| op::times),
            map(tag("/") ,  |_| op::divide),
            map(tag("%") ,  |_| op::modulo),
            )),
    )(input);
    match sign{
        Ok(v) => return v,
        Err(e) => ("error", op::unknown),
    }
}

//fn 

fn main() {
//    println!("output: {:?}", digit1("2"));
//    println!("{:?}", BoxInput("  1+1"));
    println!("{:?}", BoxInput("1+   a"));
//    let a: IResult<&str,&str> = alpha1("1+1");
 //   println!("{:?}",a);
//     match a{
//         Ok(k) => {
//             let isLetter: IResult<&str,&str> = alpha0(k.1);
//             match isLetter{
//                 Ok(r) => {
//                  let theSign = getSign(k.0);
//                  let new_string = r.1.to_string();
// //                 let VarTreelist = list::VarBox(Box::new(list::Var(new_string)), theSign.1, BoxInput(&k.0[1..]));
// //                 return Box::new(VarTreelist);
//                  println!("{:?}", theSign.1);                  
//                 }
//                 Err(l) => (),
//             }
//             }
//         ,
//         Err(e) => (),
//     }
}

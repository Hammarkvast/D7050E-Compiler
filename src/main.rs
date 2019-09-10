extern crate nom;
use nom::{
    branch::alt,
    IResult,
    bytes::complete::{tag, take_while_m_n},
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
enum list {
    Cons(Box<list>, op, Box<list>),
    Num(i32),
}

fn parser(input:&str) -> IResult<&str, &str> {
    digit1(input)
}

fn parseInput(input: &str) -> (&str, &str){
    let parsedInput = parser(input);
    match parsedInput{
        Ok(k) => k,
        Err(e) => ("i am", "error"),
    }

}

fn BoxInput(input: &str) ->Box<list>{
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

fn main() {
    println!("output: {:?}", BoxInput("20*2/2%6"));
}

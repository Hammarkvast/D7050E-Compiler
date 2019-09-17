extern crate nom;
use nom::{
    branch::alt,
    IResult,
    character::is_alphanumeric,
    character::complete::alpha1,
    bytes::complete::{tag, take_while_m_n, take_while, take_until},
    combinator::map,
    sequence::{tuple, preceded, delimited},
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
    unspecified,
}

#[derive(Debug)]
enum variable{
    Name(String),
    Value(String),
}

#[derive(Debug)]
enum list {
    Cons(Box<list>, op, Box<list>),
    VarBox(Box<list>, op, Box<list>),
    Declared(Box<variable>, types, Box<variable>),
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

fn let_parser(input: &str) -> &str{
    let test: IResult<&str, &str> = preceded(
        multispace0, 
        delimited(
            tag("let"), 
            take_until(";"), 
            tag(";")))(input);
    match test{
        Ok(k) => return k.1,
        Err(e) => "error",
    }
//    return test;
}

fn get_type(input: &str) -> (&str, types) {
    let the_type: IResult<&str, types> = preceded(
        multispace0,
        alt((
            map(tag("i32"), |_| types::integer32),
            map(tag("bool"), |_| types::boolean),
        )),
        )(input);
        println!("{:?}", the_type);
        match the_type{
            Ok(k) => return k,
            Err(e) => ("error", types::unspecified),
        }
}

fn get_type_string(input: &str) -> (&str, &str){
    let type_result: IResult<&str, &str> = preceded(
        multispace0,
        delimited(
            tag(":"), 
            take_until("="), 
            tag("=")))(input);
    match type_result{
        Ok(k) => return k,
        Err(e) => ("error", "type error"),
    }
}

fn get_value_string(input: &str) -> &str{
    let value_result: IResult<&str, &str> = preceded(
        multispace0,
        delimited(
            tag("="), 
            take_until(";"),
            tag(";")))(input);
    match value_result{
        Ok(k) => return k.0,
        Err(e) => "error, value not found"
    }
}

fn let_box(input: &str) -> Box<list>{
    let test = let_parser(input);
    let (rest_string, name_string) = parseInput(test);
    let name = name_string.to_string();
    let (value_string, type_string) = get_type_string(rest_string);
    let (empty_string, real_type) = get_type(type_string);
    let value = value_string.to_string();
//    println!("{:?}", var_string);
    let Declared_list = list::Declared(Box::new(variable::Name(name)), real_type, Box::new(variable::Value(value)));
    return Box::new(Declared_list);
}

fn BoxInput(input: &str) ->Box<list>{
     match parserTest(input){
        Ok(k) => {
            let isLetter: IResult<&str,&str> = alpha1(k.1);
            match isLetter{
                Ok(r) => {
                    println!("{:?}", k.0);
                    println!("{:?}", r.1);
                    let theSign = getSign(k.0);
                    let new_string = r.1.to_string();
                    if(k.0 == ""){
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


fn main() {
    println!("{:?}", let_box("let a:i32 = 5;"))
//    println!("output: {:?}", digit1("2"));
//    println!("{:?}", BoxInput("  1+1"));
//    println!("{:?}", BoxInput("1+   a+1"));
//    println!("{:?}", get_value_string(" = 5;"))
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

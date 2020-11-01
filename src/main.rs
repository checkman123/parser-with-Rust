use std::env;
use std::fs::File;
use std::io::prelude::*;

/* Character classes */
const LETTER: i32 = 0;
const DIGIT: i32 = 1;
const UNKNOWN: i32 = 99;

/* Token codes */
const ID: i32 =     10;
const ASSIGN: i32 = 11;
const POINT: i32 =  12;
const LPAREN: i32 = 13;
const RPAREN: i32 = 14;
const NUM: i32 =    15;
const COMMA: i32 =  16;
const SEMICOLON: i32 = 17;
const PERIOD: i32 = 18;

// -> return type of the function
fn main() -> std::io::Result<()>{

    //Variables
    let args: Vec<String> = env::args().collect();
    let mut char_type = 0;
    let mut token = 0;
    let mut lexemes =String::from("");

     //array
     let mut token_list = vec![0; 0];
     let mut num_list = vec![0; 0];
  
    //check for correct parameter
    if args.len() == 3 && args[2].len() == 2 {
        println!("Correct number of parameters!");
        println!("File = {}, Flags = {}", args[1], args[2]);

        //Check for valid flag  
        if args[2] == "-s" || args[2] == "-p"{
            

            //read file
            let filename = &args[1];
            let mut file = File::open(filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            println!("; Processing Input File: {}", filename);

            //remove whitespace
            let input = contents.replace(" ","").replace("\n","");

            let mut input_iter = input.chars();

            while let Some(mut ch) = input_iter.next() 
            {
                //clear lexeme
                lexemes = "".to_string();
                
                char_type = checkChar(ch);

                if char_type == LETTER{//LETTER
                    while char_type == LETTER || char_type == DIGIT {
                        lexemes.push(ch);
                        let next_char = input_iter.next();
                        match next_char {
                            // next character exist
                            Some(x) => ch = x,
                            // next character not exist
                            None    => (),
                        }
                        
                        char_type = checkChar(ch);
                        if char_type != LETTER && char_type != DIGIT {
                            if lexemes == "point"{
                                token = POINT
                            }else{
                                token = ID;
                            }

                            token_list.push(token);
                            
                            //clear lexeme
                            lexemes = "".to_string();
                            break;
                        }
                    }
                    token = ID;
                }

                if char_type == DIGIT{//DIGIT
                    while char_type == DIGIT {
                        lexemes.push(ch);

                        

                        let next_char = input_iter.next();
                        match next_char {
                            // next character exist
                            Some(x) => ch = x,
                            // next character not exist
                            None    => (),
                        }
                        
                        char_type = checkChar(ch);
                        if char_type != DIGIT {
                            token = NUM;
                            token_list.push(token);

                            //convert string to int and push to the array
                            let convert = lexemes.parse::<i32>().unwrap();
                            num_list.push(convert);
                            
                            //clear lexeme
                            lexemes = "".to_string();
                            break;
                        }
                    }
                    token = NUM;
                }

                if char_type == UNKNOWN{//UNKNOWN
                    if ch == '('{
                        lexemes.push(ch);
                        token = LPAREN;
                    }else if ch == ')'{
                        lexemes.push(ch);
                        token = RPAREN;
                    }else if ch == ','{
                        lexemes.push(ch);
                        token = COMMA;
                    }else if ch == '.'{
                        lexemes.push(ch);
                        token = PERIOD;
                    }else if ch == ';'{
                        lexemes.push(ch);
                        token = SEMICOLON;
                    }else if ch == '='{
                        lexemes.push(ch);
                        token = ASSIGN;
                    }
                }

                if lexemes == "point"{
                    token = POINT
                }
                token_list.push(token);
            }

            checkSyntax(&token_list);

            //All is well, everything passed the test
            println!("; Lexical and Syntax analysis passed");

            //Output either -s || -p
            if args[2] == "-s"{
                println!("(calculate-triangle (make-point {} {}) (make-point {} {}) (make-point {} {}))", 
                num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
            }else{// for "-p"
                println!("query(line(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(triangle(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(vertical(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(horizontal(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(equilateral(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(isosceles(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(right(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(scalene(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(acute(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("query(obtuse(point2d({},{}), point2d({},{}), point2d({}, {})))",num_list[0],num_list[1],num_list[2],num_list[3],num_list[4],num_list[5]);
                println!("writeln(T) :- write(T), nl.
                main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),
                      halt.");
            }
        }else{
            panic!("Command Error: Wrong flag. [-s|-p]")
        }
    }else{
        panic!("Command Error: Please input command correctly: cargo run filename.txt [-s|-p]")
    }

    Ok(())

}

//check the character is digit, letter, unknown, or unvalid
fn checkChar(ch: char)->i32{
    if ch.is_alphabetic(){
        return LETTER;
    }
    if ch.is_digit(10){
        return DIGIT;
    }
    if ch == '(' || ch == ')' || ch == '.' || ch == ';' || ch == ',' || ch == '='{// UNKNOWN -> ( | ) | , | . | ; | =
        return UNKNOWN;
    }
    panic!("Lexical Error: character is not valid: {}", ch);
}


/*
START     --> POINT_DEF; POINT_DEF; POINT_DEF.
POINT_DEF --> ID = point(NUM, NUM)
ID        --> LETTER+
NUM       --> DIGIT+
LETTER    --> a | b | c | d | e | f | g | ... | z
DIGIT     --> 0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9

Grammar -> ID = point(NUM, NUM); ID = point(NUM, NUM); ID = point(NUM, NUM).
*/
fn checkSyntax(token_list: &[i32]){
    let mut counter = 0;
    //ID==========================
    if ID == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // =
    if ASSIGN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // POINT
    if POINT == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // (
    if LPAREN == token_list[counter]{

        counter += 1;
    } else {error();}
    // num
    if NUM == token_list[counter]{
        
        counter += 1;
    } else {error();}
    // ,
    if COMMA == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // num
    if NUM == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // )
    if RPAREN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // ;
    if SEMICOLON == token_list[counter]{
         
        counter += 1;
    } else {error();}

    //ID==========================
    if ID == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // =
    if ASSIGN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // POINT
    if POINT == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // (
    if LPAREN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // num
    if NUM == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // ,
    if COMMA == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // num
    if NUM == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // )
    if RPAREN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // ;
    if SEMICOLON == token_list[counter]{
         
        counter += 1;
    } else {error();}

    //ID==========================
    if ID == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // =
    if ASSIGN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // POINT
    if POINT == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // (
    if LPAREN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // num
    if NUM == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // ,
    if COMMA == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // num
    if NUM == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // )
    if RPAREN == token_list[counter]{
         
        counter += 1;
    } else {error();}
    // .
    if PERIOD == token_list[counter]{
         
        counter += 1;
    } else {error();}
}

fn error(){
    panic!("Syntax Error");
}
use std::collections::HashSet;

#[derive(Debug)]
enum TokenType{
    tok_identifier,
    tok_number,
    tok_operator,
    tok_seperator,
    tok_eof,
  }


#[derive(Debug)]  
struct Tokenizer{
    val : String,
    token : TokenType  
  }

pub fn lexer( source : &str ) -> Vec<Tokenizer> {

let keywords : HashSet<&str> =  HashSet::from([ "int", "float", "double", "char", "if", "else", "while", "for", "return" ]);
let operators: HashSet<char> = HashSet::from (['=', '+', '-', '/', ',', '&']);
let mut tokens : Vec<Tokenizer> = Vec::new();

 let mut c = source.chars().peekable();

 while let Some(curr_char) = c.next(){
    
  if curr_char.is_whitespace(){
        continue;
    }

  if curr_char.is_alphabetic(){
    let mut identifier : String = String::new();
    identifier.push(curr_char);

    while let Some(&next) = c.peek(){
      if next.is_alphanumeric() || next == '_' {
        identifier.push(next);
        c.next();
        } else {
            break;
        }
    } 

    if keywords.contains( identifier.as_str() ){
        tokens.push( Tokenizer{ val: identifier, token: TokenType::tok_identifier } );
    } 
  }

   if curr_char.is_numeric(){
      let mut number = String::new();
      number.push(curr_char);

      while let Some(&next) = c.peek(){
        if next.is_numeric() || next == '.' {
            number.push(next);
            c.next();
        } else {
            break;
        }
      }
     tokens.push( Tokenizer{ val : number, token: TokenType::tok_number } );
   } 
   
  }
 return tokens;
}  

pub fn main(){
  
  let a = "int main if() else int i = 0.3"; 
  let x = lexer(a);

  for y in x{
    println!("{:?}", y);
  }
 
}
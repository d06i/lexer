use std::collections::HashSet;

#[derive(Debug)]
enum TokenType{
    TokIdentifier,
    TokNumber,
    TokOperator,
    TokDoubleOperator,
    TokSeperator, 
  }

#[derive(Debug)]  
struct Tokenizer{
    val : String,
    token : TokenType  
  }

fn lexer( source : &str ) -> Vec<Tokenizer> {

  let keywords : HashSet<&str> =  HashSet::from([ "int", "float", "double", "char", "if", "else", "while", "for", "return" ]);
  let double_operators : HashSet<&str> =  HashSet::from([  "==", "!=", "<=", ">=", "++", "--", "&&", "||", "<<", ">>" ]);
  let operators: HashSet<char> = HashSet::from (['=', '+', '-', '/', ',', '&', ';', '!']);
  let seperators: HashSet<char> = HashSet::from ([ '{', '}', '(', ')', '[', ']' ]);
  let mut tokens : Vec<Tokenizer> = Vec::new();

 let mut c = source.chars().peekable();

 while let Some(curr_char) = c.next(){
    
    // skip spaces
   if curr_char.is_whitespace(){
          continue;
      }

    // operators
   else if operators.contains(&curr_char){
      let mut op_temp: String = curr_char.to_string();
      if let Some(&next) = c.peek(){
        op_temp.push(next);
        if( double_operators.contains( op_temp.as_str() ) ){
          tokens.push( Tokenizer{ val: op_temp, token: TokenType::TokDoubleOperator  });
        }
        tokens.push( Tokenizer{ val: curr_char.to_string(), token: TokenType::TokOperator } ); 
      }
       
    }  

    // seperators
   else if seperators.contains(&curr_char){
      tokens.push( Tokenizer{ val: curr_char.to_string(), token: TokenType::TokSeperator } );
   }

  // identifiers  
   else if curr_char.is_alphabetic(){
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

      // check identifier
      if keywords.contains( identifier.as_str() ){
          tokens.push( Tokenizer{ val: identifier, token: TokenType::TokIdentifier } );
      } 
    }
    
   // numbers 
   else if curr_char.is_numeric(){
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
     tokens.push( Tokenizer{ val : number, token: TokenType::TokNumber } );
   } 

   // unknowns chars
   else {
     println!("Unknown token -> {}", curr_char);
   }
   
  }

 return tokens;
}  

pub fn main(){
  
  let a = " int main if() else int i = 0.3; int x == (i + 3)-6; if (x != i) {x = i;}"; 
  let x = lexer(a);

  for y in x{
    println!("{:?}", y);
  }
 
}
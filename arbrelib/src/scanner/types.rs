// Written by Luke Bhan, 3/21/2020
// Serves as a lexer for a string of cli arguments
//
//
#[derive(PartialEq, Clone, Debug)]
pub enum TokenType{
    // arbre to recognize and arbre command
    Arbre,

    // add command
    Add, 

    // init command
    Init, 

    // Remove command
    Remove,

    // Commit Command
    Commit, 

    // Push Command
    Push, 

    // may add more commands later

    // when given an invalid token
    Invalid(String)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token{
    pub token_type: TokenType
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_eq_1(){
        let t = Token{token_type: TokenType::Arbre};
        let r = Token{token_type: TokenType::Arbre};
        assert_eq!(true, t==r);
    }

    #[test]
    fn test_eq_2(){
        let t = Token{token_type: TokenType::Invalid("test".into())};
        let r = Token{token_type: TokenType::Invalid("test".into())};
        assert_eq!(true, t==r);
    }
    #[test]
    fn test_eq_3(){
        let t = Token{token_type: TokenType::Invalid("".into())};
        let r = Token{token_type: TokenType::Invalid("".into())};
        assert_eq!(true, t==r);
    }

    #[test]
    fn test_neq_1(){
        let t = Token{token_type: TokenType::Push};
        let r = Token{token_type: TokenType::Commit};
        assert_ne!(true, t==r);
    }

    #[test]
    fn test_neq_2(){
        let t = Token{token_type: TokenType::Invalid("s".into())};
        let r = Token{token_type: TokenType::Invalid("".into())};
        assert_ne!(true, t==r);
    }

    #[test]
    fn test_neq_3(){
        let t = Token{token_type: TokenType::Invalid("S".into())};
        let r = Token{token_type: TokenType::Invalid("s".into())};
        assert_ne!(true, t==r);
    }
}

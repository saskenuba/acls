use crate::syntax::SyntaxKind::{DEFN_KW, NS_KW};

#[derive(Debug)]
enum SyntaxKind {
    NS_KW,
    DEFN_KW,
}

impl SyntaxKind {
    fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "ns" => NS_KW,
            "defn" => DEFN_KW,
            _ => return None,
        };

        Some(kw)
    }
}



enum RequireKind {


}
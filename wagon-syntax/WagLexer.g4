lexer grammar WagLexer;

// LEXER GRAMMAR
//META-DATA
METADELIM          : '='*;
INCLUDEDECL        : '::';

IDENTIFIER         : [a-zA-Z][a-zA-Z-2-9_];
ATTRSPEC           : '$' | '*' | '&';

BOOL               : 'true' | 'false';
NUM                : [0-9]*;
FLOAT              : [0-9]*'.'[0-9]*;
STRING             : '"' [^']* '"'
                   |  '\'' [^']* '\'';

EBNFTYPE           : '+' | '*' | '?';
EBNF_OPTION        : '|';

TERMINAL           : '/' [^/] '/'; // Regex
SUBPROC            : '$( /[^)]*/ )';

DECLARATION        : ':';
STATEMENT_END      : ';';
ASSIGN_VALUE       : '=';
INVERSE_SIGN       : '!';
BRACKET_L          : '[';
BRACKET_R          : ']';
ATTR_DECL_L        : '<';
ATTR_DECL_R        : '>';
ATTR_ASSIGN_L      : '{';
ATTR_ASSIGN_R      : '}';
LPAREN             : '(';
RPAREN             : ')';
DELIMITER          : ',';

IF                 : 'if';
ELSE               : 'else';
THEN               : 'then';
AND                : '&&';
OR                 : '||';

SUMOP              : '+' | '-';
FACTORIAL          : '**';
TERMOP             : '*' | '//' | '/' | '%';
COMPOP             : '<' | '<=' | '>' | '>=' | '==' | '!=' | 'in';

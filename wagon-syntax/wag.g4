grammar wag;

wag : metadata? rule*;

// PARSER GRAMMAR

// Metadata Section

metadata           : meta* META_END;

meta               : include | config;

include            : IDENTIFIER? INCLUDEDECL IDENTIFIER include?;

config             : IDENTIFIER DECLARATION IDENTIFIER STATEMENT_END;

//comment            : COMMENT_START COMMENT_CONTENT COMMENT_END | COMMENT_LINE COMMENT_CONTENT;

// Production Rules

rule               : IDENTIFIER nTArgs? PROPESITION_SIGN rhs STATEMENT_END;

rhs                : weight? chunk* EBNF_OPTION rhs | weight? chunk* ;

weight             : BRACKET_L attrIdentifier? expression BRACKET_R;

chunk              : chunkP EBNFTYPE?;

chunkP             : symbol

	               |  LPAREN chunk* RPAREN

	               ;

symbol             :  nonTerminal

                   |  terminal

                   |  assignment

                   ;

terminal : STRING | FLOAT | BOOL | NUM ;

nonTerminal        : IDENTIFIER EBNFTYPE?;

nTArgs             : LEFT_ANGLE_BRACKET attrIdentifierList RIGHT_ANGLE_BRACKET;

attrIdentifier     : ATTRSPEC? IDENTIFIER;

attrIdentifierList : attrIdentifier DELIMITER attrIdentifierList

                   |  attrIdentifier

                   |  STRING

                   ;

assignment         : nTAssigment | attributeAssignment;
nTAssigment        : nonTerminal nTArgs EBNFTYPE?;
attributeAssignment: ATTR_ASSIGN_L (attrIdentifier EQ expression STATEMENT_END)* ATTR_ASSIGN_R;

// ATTRibute Evaluation

expression         : SUBPROC
                   |  if
                   |  disjunct
                   ;

if                 : IF disjunct THEN disjunct (ELSE expression)?;

disjunct           : conjunct (AND disjunct)?;

conjunct           : inverse (OR conjunct)?;

inverse            : INVERSE_SIGN? comparison;

comparison         : sum (compop sum)?;

sum                : term sumP?;

sumP               : SUMOP term sumP?;

term               : factor termP?;

termP              : TERMOP factor termP?;

factor             : atom (FACTORIAL factor)?;

atom               : attrIdentifier

                   |  dictonary

                   |  BOOL

                   |  NUM*

                   |  FLOAT

                   |  STRING

                   |  LPAREN expression RPAREN

                   ;

dictonary :  BRACKET_L expression BRACKET_R;

compop             : LEFT_ANGLE_BRACKET | LEQ | RIGHT_ANGLE_BRACKET | GEQ | EQ2 | NEQ | IN;

// Lexer

INCLUDEDECL        : '::';

PROPESITION_SIGN   : '->';

IDENTIFIER         : [a-zA-Z][a-zA-Z0-9_]*;

TERMOP             : '*' | '//' | '/' | '%';

ATTRSPEC           : '$' | '*' | '&';

SUMOP              : '+' | '-';

EBNFTYPE           : '@' | '~' | '?';

BOOL               : 'true' | 'false';

NUM                : [0-9];
wa
FLOAT              : [0-9]*'.'[0-9]*;

STRING             : '"' ~('\r' | '\n' | '"')* '"'

                   | '\'' ~('\r' | '\n' | '"')* '\'';

EBNF_OPTION        : '|';

SUBPROC            : '$( /[^)]*/ )';

DECLARATION        : ':';

STATEMENT_END      : ';';

EQ                 : '=';

META_END           : '****';

INVERSE_SIGN       : '!';

BRACKET_L          : '[';

BRACKET_R          : ']';

ATTR_ASSIGN_L      : '{';

ATTR_ASSIGN_R      : '}';

LPAREN             : '(';

RPAREN             : ')';

LEFT_ANGLE_BRACKET : '<';

RIGHT_ANGLE_BRACKET: '>';

DELIMITER          : ',';

IF                 : 'if';

ELSE               : 'else';

THEN               : 'then';

AND                : '&&';

OR                 : '||';

FACTORIAL          : '**';

LEQ                : '<=';

GEQ                : '>=';

EQ2                : '==';

NEQ                : '!=';

IN                 : 'in';


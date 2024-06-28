parser grammar wag;

options {
    tokenVocab = WagLexer;
}

wag : metadata? rule*;

// PARSER GRAMMAR
// Metadata Section
include            : IDENTIFIER? INCLUDEDECL IDENTIFIER include?;
config             : IDENTIFIER DECLARATION expression STATEMENT_END;
meta               : include | config;
metadata           : meta* METADELIM;

// Production Rules
rule               : IDENTIFIER nTArgs? DECLARATION rhs;
rhs                : weight? chunk* EBNF_OPTION rhs |  weight? chunk* ;
weight             : BRACKET_L expression BRACKET_R;
chunk              : chunkP EBNFTYPE?;
chunkP             : symbol
 	               |  LPAREN chunk* RPAREN
 	               ;
symbol             :  nonTerminal
                   |  TERMINAL
                   |  assignment
                   ;

nonTerminal        : IDENTIFIER EBNFTYPE?;
nTArgs             : ATTR_DECL_L attrIdentifierList ATTR_DECL_R;
attrIdentifier     : ATTRSPEC? IDENTIFIER;

attrIdentifierList : attrIdentifier DELIMITER attrIdentifierList
                   |  attrIdentifier
                   |  STRING
                   ;
assignment         : ATTR_ASSIGN_L (attrIdentifier ASSIGN_VALUE expression STATEMENT_END)* ATTR_ASSIGN_R;

// ATTRibute Evaluation
expression         : SUBPROC
                   |  if
                   |  disjunct
                   ;
if                 : IF disjunct THEN disjunct (ELSE expression)?;
disjunct           : conjunct (AND disjunct)?;
conjunct           : inverse (OR conjunct)?;
inverse            : INVERSE_SIGN? comparison;
comparison         : sum (COMPOP sum)?;
sum                : term sumP?;
sumP               : SUMOP term sumP?;
term               : factor termP?;
termP              : TERMOP factor termP?;
factor             : atom (FACTORIAL factor)?;
atom               : attrIdentifier
                   |  dictonary
                   |  BOOL
                   |  NUM
                   |  FLOAT
                   |  STRING
                   |  LPAREN expression RPAREN
                   ;

dictonary : attrIdentifier BRACKET_L expression BRACKET_R;



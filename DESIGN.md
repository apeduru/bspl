`bspl` is split into three phases: lexer, parser, and evaluator.

The lexer is responsible for creating a token stream from the REPL input.
It does not concern itself with the semantics of the language. It is only interested
in understanding the syntax and classifying the scanned tokens. I followed a
similar formula that Python's tokenize module uses. Take for example the
following command:

`echo "12 | ( 1 << 12 )" | /usr/bin/python -m tokenize`

Produces the output:
~~~
1,0-1,2:	NUMBER	'12'
1,2-1,3:	OP	'|'
1,3-1,4:	OP	'('
1,4-1,5:	NUMBER	'1'
1,5-1,7:	OP	'<<'
1,7-1,9:	NUMBER	'12'
1,9-1,10:	OP	')'
1,10-1,11:	NEWLINE	'\n'
2,0-2,0:	ENDMARKER	''
~~~

The parser is responsible for pruning and creating a structure out
of the raw token stream. As a result, the parser contains stricter rules and more
edge cases to deal with it.  Some of the easier token analysis work of the parser
may have been alleviated had these rules been shifted earlier to the lexer.
Utilizes the Shunting Yard Algorithm, the parser will create

~~~
| Precedence |   Operator  | Symbol | Associativity |
|------------|-------------|--------|---------------|
|      1     |Open Bracket |   (    | Left-to-Right |
|      1     |Close Bracket|   )    | Left-to-Right |
|      2     |NOT          |   ~    | Right-to-Left |
|      2     |Unary Minus  |   -    | Right-to-Left |
|      3     |Right Shift  |   >>   | Left-to-Right |
|      3     |Left Shift   |   <<   | Left-to-Right |
|      4     |AND          |   &    | Left-to-Right |
|      5     |XOR          |   ^    | Left-to-Right |
|      6     |OR           |   |    | Left-to-Right |
~~~

_A language that doesn't affect the way you think about programming, is not worth knowing._

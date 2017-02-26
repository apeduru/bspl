`bspl` is split into three phases: lexer, parser, and evaluator.

The lexer is responsible for creating a token stream from the REPL input.
It does not concern itself with the semantics of the language. It is only
interested in understanding the syntax and classifying the characters into
tokens. I followed a similar pattern that Python's tokenize module uses. Take
for example the following command:

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

The parser takes the raw token stream produced by the lexer, and further prunes a
suitable structure. In computer languages this often results in a parse tree, an 
AST, etc. For the purposes of this project, I used Dijkstra's [shunting yard
algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm).

The shunting-yard algorithm parses an infix mathematical expression
(the normal way to write an equation) and produces an expression in postfix
notation (also known as Reverse Polish notation). A compiler can better evaluate
expressions of this form. Using the same example from before, the postfix
expression after applying the algorithm is shown below. Note the lack of brackets.

`12 1 12 << |`

Recalling the use of BEDMAS, the allowable operators in `bspl` must have a defined
[precedence](https://en.wikipedia.org/wiki/Order_of_operations) and
[associativity](https://en.wikipedia.org/wiki/Operator_associativity) when
determining which expressions to evaluate first. The table below summarizes the
allowable operators.
~~~
| Precedence |   Operator  | Symbol | Associativity |
|------------|-------------|--------|---------------|
|      1     |Open Bracket |   (    | Left-to-Right |
|      1     |Close Bracket|   )    | Left-to-Right |
|      2     |NOT          |   ~    | Right-to-Left |
|      3     |Right Shift  |   >>   | Left-to-Right |
|      3     |Left Shift   |   <<   | Left-to-Right |
|      4     |AND          |   &    | Left-to-Right |
|      5     |XOR          |   ^    | Left-to-Right |
|      6     |OR           |   |    | Left-to-Right |
~~~

Finally the evaluator, well, evalutates the tokens in postfix notation using a
simple [algorithm](https://en.wikipedia.org/wiki/Reverse_Polish_notation#Postfix_algorithm).
This is where things get interesting. The purpose of `bspl` is to provide a 
inside view into the execution of a particular bitwise expression.
// TODO: Finish the rest



> _A language that doesn't affect the way you think about programming, is not worth knowing._

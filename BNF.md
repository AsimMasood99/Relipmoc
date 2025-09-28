**root-list**            -> root | root root-list<br>
**root**                 -> variable-declaration | function-statement

**variable-declaration**    -> type T_IDENTIFIER T_ASSIGNMENT_OPR expression T_SEMICOLON<br>

**function-statement**    -> T_FUNCTION function-type T_IDENTIFIER T_ROUND_BRACKET_OPEN params T_ROUND_BRACKET_CLOSE block<br>
**function-type**       -> type | T_VOID<br>
**type**                 -> T_INT | T_STRING | T_FLOAT | T_BOOL

**params**               -> param | param T_COMMA params | ε<br>
**param**                -> type T_IDENTIFIER

**block**                -> T_CURLY_BRACKET_OPEN statements T_CURLY_BRACKET_CLOSE<br>
**statements**                -> statement statements | ε<br>
**statement**                 -> for-statement | if-statement | while-statement | return-statement | variable-declaration | expression T_SEMICOLON

**for-statement**             -> T_FOR T_ROUND_BRACKET_OPEN init_loop_var loop_condition update_loop_var T_ROUND_BRACKET_CLOSE block<br>
**init_loop_var**                 -> variable-declaration | ε<br>
**loop_condition**                 -> expression | ε<br>
**update_loop_var**                 -> expression | ε

**while-statement**            -> T_WHILE T_ROUND_BRACKET_OPEN expression T_ROUND_BRACKET_CLOSE block

**if-statement**              -> T_IF if-statement-expr elif-statement else-statement

**elif-statement**        -> T_ELSE_IF if-statement-expr block elif-statement | ε

**else-statement**        -> T_ELSE block | ε

**if-statement-expr**     -> T_ROUND_BRACKET_OPEN expression T_ROUND_BRACKET_CLOSE block

**return-statement**             -> T_RETURN expression T_SEMICOLON

**expression**            -> assignment-expression

**assignment-expression**          -> bool-expression | bool-expression T_ASSIGNMENT_OPR assignment-expression

**bool-expression**            -> bitwise-or-expression | bool-expression bool-op bitwise-or-expression<br>
**bool-op**              -> T_OR_OPR | T_AND_OPR

**bitwise-or-expression**      -> bitwise-and-expression | bitwise-or-expression T_OR_OPR bitwise-and-expression

**bitwise-and-expression**     -> comparison-expression | bitwise-and-expression T_AND_OPR comparison-expression

**comparison-expression**            -> shift-expression | comparison-expression comparison-op shift-expression<br>
**comparison-op**              -> T_LESS_THAN_OPR | T_GREATER_THAN_OPR | T_EQUALS_OPR | T_NOT_EQUALS_OPR | T_LESS_THAN_EQUAL_TO_OPR | T_GREATER_THAN_EQUAL_TO_OPR

**shift-expression**           -> add-expression | shift-expression shift-op add-expression<br>
**shift-op**             -> T_LEFT_SHIFT_OPR | T_RIGHT_SHIFT_OPR

**add-expression**             -> mul-expression | add-expression add-op mul-expression<br>
**add-op**               -> T_PLUS_OPR | T_MINUS_OPR

**mul-expression** -> exp-expression | mul-expression mul-op exp-expression<br>
**mul-op**               -> T_MULTIPLY_OPR | T_DIVIDE_OPR | T_MODULO_OPR

**exp-expression**             -> unary-expression | unary-expression T_POWER_OPR exp-expression

**unary-expression**           -> primary | unary-op unary-expression<br>
**unary-op**             -> T_MINUS_OPR | T_NOT_OPR

**primary**              -> T_CONST_INT | T_CONST_FLOAT | T_STRINGLIT | T_CONST_BOOL | T_IDENTIFIER | T_ROUND_BRACKET_OPEN expression T_ROUND_BRACKET_CLOSE | function-call<br>

**function-call**              -> T_IDENTIFIER T_ROUND_BRACKET_OPEN function-args T_ROUND_BRACKET_CLOSE<br>
**function-args**              -> expression | expression T_COMMA function-args | ε
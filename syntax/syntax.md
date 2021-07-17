# Syntax

The syntax of Liom is Rust-like. It contains only statements and expressions for the moment. Here is a representation of
the syntax. See [here](../fuzz/corpus/main/example) for a tour of the syntax.

## Source

> Source :
>
> &nbsp;&nbsp; [Statement](#statements)*

## Statements

> Statement :
>
> &nbsp;&nbsp; [VariableDefinition](#variable-definitions)\
> &nbsp;| [Expression](#expressions)

### Variable definitions

> VariableDefinition :
>
> &nbsp;&nbsp; `let` [Identifier](tokens.md#identifier) `=` [Expression](#Expressions)

## Expressions

> Expression :
>
> &nbsp;&nbsp; [Literal](#literals)\
> &nbsp;| [VariableReference](#variable-references)\
> &nbsp;| [InfixExpression](#infix-expressions)\
> &nbsp;| [PrefixExpression](#prefix-expressions)

### Literals

> Literal :
>
> &nbsp;&nbsp; [Integer](tokens.md#integer)

### Variable references

> VariableReference :
>
> &nbsp;&nbsp; [Identifier](tokens.md#identifier)

### Infix expressions

> InfixExpression :
>
> &nbsp;&nbsp; [Expression](#expressions) [InfixOperator](tokens.md#infix-operator) [Expression](#expressions)

### Prefix expressions

> PrefixExpression :
>
> &nbsp;&nbsp; [PrefixOperator](tokens.md#prefix-operator) [Expression](#expressions)

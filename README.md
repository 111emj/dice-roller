# Script for Simulating Dice Rolls for any Tabletop game

## Syntax
- dice are notated as [q]d[n] where n represents the number of sides on the die and q represents the number of dice (by default dice results are summed)
- dice rolls can be sorted with ^ for descending order and _ for ascending order following this with a number (n) will keep the n highest or n lowest rolls respectively
- arithmetic can be performed on the results of dice rolls 
- anything inside parentheses will be reduced first
- prefacing an expression with [n]x will repeat the expression n times
- dice rolls can be indexed by placing a list of numbers inside curly-brackets after the die or sort expression.
- dice rolls can be specified to reroll values by appending rr[n] after the expression to reroll any rolls matching n

## To-Do
- iterate over dice rolls

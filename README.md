# Booleanomial
Turn boolean operations into polynomials

## What is a booleanomial?

* "booleanomial" is a portmanteau of "boolean" and "polynomial".
  
* A booleanomial is a boolean expression, such as `(a OR b) AND c`, expressed as a polynomial (of 3 variables in this case).
  
* A value of 1 corresponds to true, and a value of 0 corresponds to false.
  
* For example:
  
  * The booleanomial of `a AND b` is `y = ab`.
    If either a = 0 or b = 0, then y = 0.
    If a and b are both 1, then y = 1.
  * The booleanomial of `a OR b` is `y = -ab + a + b`. 
    If either a = 1 or b = 1, then y = 1, and otherwise y = 0.
    
## I wanna see stuff

`cargo run --example demo`

## Todo

Display order: display terms in degree/alphabetical order.
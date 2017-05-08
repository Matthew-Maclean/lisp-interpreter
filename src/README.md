# lisp-interpreter

A (very) minimal lisp repl in rust.  
Contains only eight built-in functions:

### `quote`

Returns it's argument un-evaluated. Abbreviated with the quote (`'`) character.  
Example:

    (quote a) => a
    'a        => a

### `atom`

Returns true (the atom `t`) if it's argument is an atom, otherwise false (`()`).  
Example:

    (atom 'a)  => t
    (atom '()) => ()

### `eq`

Returns true if it's arguments are both the same atom, or both the empty list.  
Example:

    (eq 'a 'a)   => t
    (eq 'a 'b)   => ()
    (eq 'a '())  => ()
    (eq '() '()) => t

### `car`

Returns the first element of it's argument (expects that argument to be a list).  
Example:

    (car '(a b c)) => a

### `cdr`

Returns the last elements (aka everything except the first) of it's argument (expects that argument to be a list).  
Example:

    (cdr '(a b c)) => (b c)

### `cons`

Prepends it's first argument to it's second (expects that it's second argument is a list).  
Example:

    (cons 'a '(b c)) => (a b c)

### `cond`

Takes a list of pairs as it's arguments. Goes through each pair, evaluating the first element, until ones returns an atom. Then it evaluates that pair's last element, and returns the value.  
Example:

    (cond ('t 'a) ('() 'b)) => a
    (cond ('() 'a) ('t 'b)) => b

### `lambda`

Takes a list of arguments as a list of atoms, and a body, as any expression. Creates a function that when passed arguments executes it's body after replacing instances of it's argument names with the result of evaluating the matching argument.  
Example:

    (lambda (x) (cons x '(b c)))    => ..
    ((lambda (x) (cons x '(b c)) a) => (a b c)

### `macro`

Creates a function that behaves like functions created by `lambda`, but does not evaluate it's arguments before replacing them.  
Example:

    (macro (x) (cons x '(b c)))      => ..
    ((macro (x) (cons x '(b c))) a)  => (a b c)
    ((macro (x) (cons x '(b c))) 'a) => ((quote a) b c)

### `label`

Takes a name and either a function or macro, and creates another function or macro where instances of the name inside the body are replaced with the `label` expression itself.  
Example:

    (label f (lambda (x) (cond ((atom x) (f '())) ('t 'a))))      => ..
    ((label f (lambda (x) (cond ((atom x) (f '())) ('t 'a)))) 'x) => a

---

## How to use the interface

Firstly, to exit, enter a line containing only `; exit`.  
The repl starts like:

    >>> : 

Enter your code here, comments start with `;` and last until a newline.  
Your code will not be run until the braces balance. After entering a line, you will see a count of open braces before the input. For example:

    >>> : (atom
    001 : (quote (
    003 : ))
    001 : )
    <<< : ()

Output is printed on a line starting with

    <<< :

If there is an error, the error will be diplayed like:

    err : <error>

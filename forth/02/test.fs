: print-fib dup . CR ;

: fib-init 1 print-fib 1 print-fib ;

: fib
    ( actually compute the next fib number and leave the stack as is )
    dup rot +
    print-fib
    ;

: fib-run ( n --) 
    ( 0 is the lower bound of the loop, the upper is already on the stack )
    0 DO 
        fib
    LOOP CR ;
    

( need to do this first as the loop vars need to be higher up the stack )
fib-init
10 fib-run

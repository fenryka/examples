#!/usr/bin/python

def func1 (a, b, c) :
    pass

print func1.__code__.co_argcount
print func1.__code__.co_varnames

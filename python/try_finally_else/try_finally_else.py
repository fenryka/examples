#!/usr/bin/python

import sys


def func1() :
    raise Exception

def func2() :
    print 'func2'

def here (func_) :
    try :
        print 'try'
        func_()
    except Exception :
        print 'except'
    else :
        print 'else'
    finally :
        print 'finally'


here (func1)
print ''
here (func2)

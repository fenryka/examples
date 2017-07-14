#!/usr/bin/python

class A (object) :
    def func (self) :
        if   isinstance (self, B) : print 'B'
        elif isinstance (self, C) : print 'C'
        else : print 'A'

class B (A) :
    pass

class C (A) :
    pass

a = A()
a.func()
b = B()
b.func()
c = C()
c.func()

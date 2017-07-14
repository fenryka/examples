#!/usr/bin/python

def func (**kwargs) :
    print kwargs
    try :
        print a
    except NameError :
        print "no a"

    print kwargs['a']

    print ""


func (a=1)


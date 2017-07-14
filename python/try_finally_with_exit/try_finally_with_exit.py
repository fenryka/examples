#!/usr/bin/python

import sys


def func() :
    raise Exception

try :
    func()
except Exception :
    print 'E'
    sys.exit (1)
finally :
    print 'F'

print 'D'

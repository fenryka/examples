#!/usr/bin/python

from contextlib import contextmanager

class e1 (Exception):
    pass

class e2 (Exception):
    pass

@contextmanager
def handle_exceptions():
    try:
        yield # Body of the with statement effectively runs here
    except e1 :
        print 'e1'
    except e2 :
        print 'e2'
    except Exception:
        Default_action_always_taken()

def f1() :
    print 'f1'
    raise e2

# Used as follows
with handle_exceptions():
    if True :
        f1()


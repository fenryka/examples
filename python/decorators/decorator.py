#!/usr/bin/python


def wrapper1 (func) :
    def inner(*args, **kwargs) :
        try :
            if not kwargs['space_'] : kwargs['space_'] = "default"
        except KeyError :
            kwargs['space_'] = "default"

        return func (*args, **kwargs)

    return inner

def wrapper2 (func) :
    def inner(*args, **kwargs) :
        try :
            if not kwargs['parent_'] : kwargs['parent_'] = "default"
        except KeyError :
            kwargs['parent_'] = "default"

        return func (*args, **kwargs)

    return inner

@wrapper1
@wrapper2
def wrap_me (args_, space_ = None, parent_ = None) :
    print "space  = %s" % str (space_)
    print "parent = %s" % str (parent_)
    print ""


def wrapper3 (func) :
    def wrap (*args, **kwargs) :
        pants = 'hello'
        print ""
        print __name__
        print locals()
        func (*args, **kwargs)

    return wrap

@wrapper3
def wrap_me_2() :
    print ""
    print __name__
    print locals()
    print pants

wrap_me ('a')
wrap_me ('b', space_  = "1")
wrap_me ('b', parent_ = "1")
wrap_me ('b', space_ = 2, parent_ = "1")
wrap_me ('b', space_ = 2, parent_ = "3")
wrap_me ('b', parent_ = 2, space_ = "3")


try :
    print locals()
    wrap_me_2()
except NameError :
    print "Sorry Kate, this doens't work the way you'd like"

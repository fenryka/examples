#!/usr/bin/env python3

import re

def find(str_) :
    print (str_)

    res = re.finditer(r"(?P<text>[^[]*)(?P<link>[^\)]*)\)|(?P<remainder>.+)", str_)

    rtn = ""
    for x in res :
        if x.group ("remainder") :
            rtn += x.group ("remainder")
        else :
            rtn += x.group ("text")
            res2 = re.match (r"\[(?P<useme>[^]]+)", x.group ("link"))
            rtn += (res2.group ("useme"))

    print (rtn + "\n")


find ("hello world")
find ("[thing](thing)")
find ("[thing](thing), [thing2](thing2)")
find ("some text [thing](thing), [thing2](thing2)")
find ("some text [thing](thing), [thing2](thing2) and then some")

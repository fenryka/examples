#!/usr/bin/python
import os

f1 = '/file/a'
f2 = '/file/a.txt'
f3 = '/file/a.txt.arse'


print str (os.path.splitext (f1))
print str (os.path.splitext (f2))
print str (os.path.splitext (f3))

print str (os.path.split (f1))
print str (os.path.split (f2))
print str (os.path.split (f3))

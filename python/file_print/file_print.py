#!/usr/bin/python

from os.path import abspath, split

print split (abspath (__file__))

from import_1 import print_from_1

print split (print_from_1())

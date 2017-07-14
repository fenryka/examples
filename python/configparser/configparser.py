#!/usr/bin/python

import ConfigParser

config = ConfigParser.ConfigParser()
config.read ('testrc')

print config.sections()
for section in config.sections() :
    print section

    print dict (config.items (section))

#!/usr/bin/python

from confluence import Confluence

import os
import subprocess

#
# The space the parent page exists within
#
_space = "<fill me in>"

#
# The parent page name
#
_page  = "<fill me in>"

confluence_server   = "<fill me in>"
confluence_user     = "<fill me in>"
confluence_password = "<fill me in>"

def connect() :
    connection = Confluence (
        url      = confluence_server,
        username = confluence_user,
        password = confluence_password)

    return connection

wiki   = connect()
pageid = wiki.getPageId (_page, _space)

print "saving files:"
for page in wiki.getDescendents (_space, pageid) :
    lpage = wiki.getPage (page['title'], page['space'])

    with open (page['title'], 'w') as f:
        print "  %s" % page['title']
        f.write (lpage['content'])

        try :
            pages.append (page['title'])
        except NameError :
            pages = []
            pages.append (page['title'])

if os.path.exists ('wiki.tar') :
    os.remove ('wiki.tar')

if os.path.exists ('wiki.tar.bz2') :
    os.remove ('wiki.tar.bz2')

print "creating tarball"

tar_args = [ 'tar', '-cvf', 'wiki.tar' ]
tar_args.extend (pages)

out = subprocess.check_output (tar_args)
out = subprocess.check_output ([ 'bzip2', 'wiki.tar' ] )

print "cleaning up"

for page in pages :
    os.remove (page)


# Want to be able to use the new method of setting the project version
CMAKE_POLICY (SET CMP0048 NEW)

CMAKE_MINIMUM_REQUIRED (VERSION 3.10)

PROJECT (SHARED-TEST VERSION 0.0.3)

ADD_DEFINITIONS ("-Wall -g -ansi -pedantic")


#
# For "reasons" something to do with the way the command line compiler
# is invoked on MacOS /usr/local/include is removed from the 
# system include path so add it back in or we will fail to find
# qpid-python files as brew installs it there
#
if(APPLE)
    include_directories (/usr/local/include)
    link_directories (/usr/local/lib)
endif()

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

ADD_SUBDIRECTORY (lib)
ADD_SUBDIRECTORY (bin)

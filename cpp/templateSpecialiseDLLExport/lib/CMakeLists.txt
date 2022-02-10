set (libname shared-test)

set (shared_sources
    shared-test.cxx
    shared-test.h
)


ADD_LIBRARY (${libname} SHARED ${shared_sources})

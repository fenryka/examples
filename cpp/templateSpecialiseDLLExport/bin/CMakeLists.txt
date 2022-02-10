include_directories (${SHARED-TEST_SOURCE_DIR}/lib)

link_directories (${SHARED-TEST_BINARY_DIR}/lib)

set (sources
        main.cxx)


add_executable (test ${sources})

target_link_libraries (test shared-test)


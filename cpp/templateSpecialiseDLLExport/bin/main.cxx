#include <iostream>

#include "shared-test.h"

int
main (int argc, char ** argv) {

    std::cout << shared::export_me<int>() << std::endl;
    std::cout << shared::export_me<std::string>() << std::endl;

    return EXIT_SUCCESS;
}

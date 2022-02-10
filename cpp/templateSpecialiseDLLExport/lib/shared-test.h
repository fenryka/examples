#pragma once

#include <stdexcept>

namespace shared {

    template<typename T>
    [[maybe_unused]] T
    export_me() {
        throw std::runtime_error ("BAD");
    }

}

/**
 * So, if we don't put this in the header we don't seem to export the symbol for the
 * template specialisation in the shared object... which is interesting
 */
#if 1
template<> [[maybe_unused]] int32_t shared::export_me();
template<> [[maybe_unused]] std::string shared::export_me();
#endif

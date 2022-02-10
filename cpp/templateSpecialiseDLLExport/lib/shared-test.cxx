#include "shared-test.h"

#include <string>

template<>
[[maybe_unused]] int32_t
shared::export_me() {
    return 110;
}
template<>
[[maybe_unused]] std::string
shared::export_me() {
    return "Hey";
}

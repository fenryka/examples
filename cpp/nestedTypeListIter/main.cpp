#include <iostream>

template <typename T, typename ...Args>
struct
doubleTester {

    template <typename OUTER, typename T2, typename ...Args2>
    struct
    with {
        static void
        test() {
            std::cout << std::string (typeid (OUTER).name()) << "::" << std::string(typeid(T2).name()) << std::endl;
            if constexpr (!sizeof...(Args2)) {
            } else {
                with<OUTER, Args2...>::test();
            }
        }
    };

    template <typename T2, typename ...Args2>
    static void
    test() {
        std::cout << std::string(typeid (T).name())<< std::endl;

        with<T, T2, Args2...>::test();

        if constexpr (sizeof...(Args)) {
            doubleTester<Args...>::template test<T2, Args2...> ();
        }
    }
};



int main () {
    std::cout << "Hello, World!" << std::endl;
    doubleTester<int, char>::test<double, float>();
    return 0;
}

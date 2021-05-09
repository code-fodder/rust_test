#include "cutils.hpp"
#include <iostream>

int cadd(int a, int b)
{
    int res = a + b;
    std::cout << "Hello from C: " << a << " + " << b << " = " << res << std::endl;
    return res;
}

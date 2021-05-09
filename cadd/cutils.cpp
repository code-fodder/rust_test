#include "cutils.hpp"
#include <iostream>

int cadd(int a, int b)
{
    int res = a + b;
    std::cout << "res from c: " << res << std::endl;
    return res;
}

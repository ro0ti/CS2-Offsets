/*
 * https://github.com/a2x/cs2-dumper
 * 2023-10-30 15:11:09.607404700 UTC
 */

#pragma once

#include <cstddef>

namespace CAnimScriptBase {
    constexpr std::ptrdiff_t m_bIsValid = 0x8; // bool
}

namespace EmptyTestScript {
    constexpr std::ptrdiff_t m_hTest = 0x10; // CAnimScriptParam<float>
}
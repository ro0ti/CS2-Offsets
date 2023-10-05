#pragma once

#include <cstddef>

namespace client_dll {
    constexpr std::ptrdiff_t entityList = 0x178D8E8;
    constexpr std::ptrdiff_t globalVars = 0x1690D18;
    constexpr std::ptrdiff_t interfaceLinkList = 0x1973BA8;
    constexpr std::ptrdiff_t localPlayerController = 0x17DC158;
    constexpr std::ptrdiff_t localPlayerPawn = 0x187AC48;
    constexpr std::ptrdiff_t plantedC4 = 0x1880DE0;
    constexpr std::ptrdiff_t viewAngles = 0x18DA700;
    constexpr std::ptrdiff_t viewMatrix = 0x187B730;
}

namespace engine2_dll {
    constexpr std::ptrdiff_t networkGameClient = 0x48BAB0;
    constexpr std::ptrdiff_t networkGameClient_maxClients = 0x250;
    constexpr std::ptrdiff_t networkGameClient_signOnState = 0x240;
}
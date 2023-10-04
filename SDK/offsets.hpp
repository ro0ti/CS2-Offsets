#pragma once

#include <cstddef>

namespace client_dll {
    constexpr std::ptrdiff_t entityList = 0x178D8D8;
    constexpr std::ptrdiff_t globalVars = 0x1690D28;
    constexpr std::ptrdiff_t interfaceLinkList = 0x1973B98;
    constexpr std::ptrdiff_t localPlayerController = 0x17DC148;
    constexpr std::ptrdiff_t localPlayerPawn = 0x187AC38;
    constexpr std::ptrdiff_t plantedC4 = 0x1880D80;
    constexpr std::ptrdiff_t viewAngles = 0x18DA6F0;
    constexpr std::ptrdiff_t viewMatrix = 0x187B720;
}

namespace engine2_dll {
    constexpr std::ptrdiff_t networkGameClient = 0x48AAB0;
    constexpr std::ptrdiff_t networkGameClient_maxClients = 0x250;
    constexpr std::ptrdiff_t networkGameClient_signOnState = 0x240;
}
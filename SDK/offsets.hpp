#pragma once

#include <cstddef>

namespace client_dll {
    constexpr std::ptrdiff_t entityList = 0x17888D8;
    constexpr std::ptrdiff_t globalVars = 0x168BCE8;
    constexpr std::ptrdiff_t interfaceLinkList = 0x196EC98;
    constexpr std::ptrdiff_t localPlayerController = 0x17D7158;
    constexpr std::ptrdiff_t localPlayerPawn = 0x1875C48;
    constexpr std::ptrdiff_t plantedC4 = 0x187B9E0;
    constexpr std::ptrdiff_t viewAngles = 0x18D5700;
    constexpr std::ptrdiff_t viewMatrix = 0x1876730;
}

namespace engine2_dll {
    constexpr std::ptrdiff_t networkGameClient = 0x485AB0;
    constexpr std::ptrdiff_t networkGameClient_maxClients = 0x250;
    constexpr std::ptrdiff_t networkGameClient_signOnState = 0x240;
}
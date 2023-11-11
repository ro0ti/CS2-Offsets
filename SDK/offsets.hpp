/*
 * https://github.com/a2x/cs2-dumper
 * 2023-11-11 18:11:10.027377800 UTC
 */

#pragma once

#include <cstddef>

namespace ClientDll {
    constexpr std::ptrdiff_t dwBaseEntityModel_setModel = 0x583320;
    constexpr std::ptrdiff_t dwEntityList = 0x17ADAE0;
    constexpr std::ptrdiff_t dwForceBackward = 0x16B25E0;
    constexpr std::ptrdiff_t dwForceCrouch = 0x16B28B0;
    constexpr std::ptrdiff_t dwForceForward = 0x16B2550;
    constexpr std::ptrdiff_t dwForceJump = 0x16B2820;
    constexpr std::ptrdiff_t dwForceLeft = 0x16B2670;
    constexpr std::ptrdiff_t dwForceRight = 0x16B2700;
    constexpr std::ptrdiff_t dwGameEntitySystem_getBaseEntity = 0x606930;
    constexpr std::ptrdiff_t dwGameEntitySystem_getHighestEntityIndex = 0x5F8620;
    constexpr std::ptrdiff_t dwGameRules = 0x1809720;
    constexpr std::ptrdiff_t dwGlobalVars = 0x16AE498;
    constexpr std::ptrdiff_t dwGlowManager = 0x1809748;
    constexpr std::ptrdiff_t dwInterfaceLinkList = 0x1904F68;
    constexpr std::ptrdiff_t dwLocalPlayerController = 0x17FCDB8;
    constexpr std::ptrdiff_t dwLocalPlayerPawn = 0x16B9368;
    constexpr std::ptrdiff_t dwPlantedC4 = 0x1810CC8;
    constexpr std::ptrdiff_t dwViewAngles = 0x186B0F0;
    constexpr std::ptrdiff_t dwViewMatrix = 0x180C0E0;
    constexpr std::ptrdiff_t dwViewRender = 0x180C938;
}

namespace Engine2Dll {
    constexpr std::ptrdiff_t dwBuildNumber = 0x48B524;
    constexpr std::ptrdiff_t dwNetworkGameClient = 0x48AAC0;
    constexpr std::ptrdiff_t dwNetworkGameClient_getLocalPlayer = 0xF0;
    constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x250;
    constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x240;
    constexpr std::ptrdiff_t dwWindowHeight = 0x541E1C;
    constexpr std::ptrdiff_t dwWindowWidth = 0x541E18;
}
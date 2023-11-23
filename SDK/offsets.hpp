/*
 * https://github.com/a2x/cs2-dumper
 * 2023-11-23 10:28:28.612975800 UTC
 */

#pragma once

#include <cstddef>

namespace ClientDll {
    constexpr std::ptrdiff_t dwBaseEntityModel_setModel = 0x584690;
    constexpr std::ptrdiff_t dwEntityList = 0x17B5200;
    constexpr std::ptrdiff_t dwForceBackward = 0x16B57E0;
    constexpr std::ptrdiff_t dwForceCrouch = 0x16B5AB0;
    constexpr std::ptrdiff_t dwForceForward = 0x16B5750;
    constexpr std::ptrdiff_t dwForceJump = 0x16B5A20;
    constexpr std::ptrdiff_t dwForceLeft = 0x16B5870;
    constexpr std::ptrdiff_t dwForceRight = 0x16B5900;
    constexpr std::ptrdiff_t dwGameEntitySystem_getBaseEntity = 0x607BE0;
    constexpr std::ptrdiff_t dwGameEntitySystem_getHighestEntityIndex = 0x5F9910;
    constexpr std::ptrdiff_t dwGameRules = 0x1810EB0;
    constexpr std::ptrdiff_t dwGlobalVars = 0x16B1670;
    constexpr std::ptrdiff_t dwGlowManager = 0x1810ED8;
    constexpr std::ptrdiff_t dwInterfaceLinkList = 0x190E578;
    constexpr std::ptrdiff_t dwLocalPlayerController = 0x1804518;
    constexpr std::ptrdiff_t dwLocalPlayerPawn = 0x16BC598;
    constexpr std::ptrdiff_t dwPlantedC4 = 0x1818478;
    constexpr std::ptrdiff_t dwViewAngles = 0x1872890;
    constexpr std::ptrdiff_t dwViewMatrix = 0x1813840;
    constexpr std::ptrdiff_t dwViewRender = 0x18140C0;
}

namespace Engine2Dll {
    constexpr std::ptrdiff_t dwBuildNumber = 0x48B514;
    constexpr std::ptrdiff_t dwNetworkGameClient = 0x48AAC0;
    constexpr std::ptrdiff_t dwNetworkGameClient_getLocalPlayer = 0xF0;
    constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x250;
    constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x240;
    constexpr std::ptrdiff_t dwWindowHeight = 0x541DBC;
    constexpr std::ptrdiff_t dwWindowWidth = 0x541DB8;
}
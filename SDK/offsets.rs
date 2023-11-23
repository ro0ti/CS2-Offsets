/*
 * https://github.com/a2x/cs2-dumper
 * 2023-11-23 10:28:28.619642500 UTC
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod ClientDll {
    pub const dwBaseEntityModel_setModel: usize = 0x584690;
    pub const dwEntityList: usize = 0x17B5200;
    pub const dwForceBackward: usize = 0x16B57E0;
    pub const dwForceCrouch: usize = 0x16B5AB0;
    pub const dwForceForward: usize = 0x16B5750;
    pub const dwForceJump: usize = 0x16B5A20;
    pub const dwForceLeft: usize = 0x16B5870;
    pub const dwForceRight: usize = 0x16B5900;
    pub const dwGameEntitySystem_getBaseEntity: usize = 0x607BE0;
    pub const dwGameEntitySystem_getHighestEntityIndex: usize = 0x5F9910;
    pub const dwGameRules: usize = 0x1810EB0;
    pub const dwGlobalVars: usize = 0x16B1670;
    pub const dwGlowManager: usize = 0x1810ED8;
    pub const dwInterfaceLinkList: usize = 0x190E578;
    pub const dwLocalPlayerController: usize = 0x1804518;
    pub const dwLocalPlayerPawn: usize = 0x16BC598;
    pub const dwPlantedC4: usize = 0x1818478;
    pub const dwViewAngles: usize = 0x1872890;
    pub const dwViewMatrix: usize = 0x1813840;
    pub const dwViewRender: usize = 0x18140C0;
}

pub mod Engine2Dll {
    pub const dwBuildNumber: usize = 0x48B514;
    pub const dwNetworkGameClient: usize = 0x48AAC0;
    pub const dwNetworkGameClient_getLocalPlayer: usize = 0xF0;
    pub const dwNetworkGameClient_maxClients: usize = 0x250;
    pub const dwNetworkGameClient_signOnState: usize = 0x240;
    pub const dwWindowHeight: usize = 0x541DBC;
    pub const dwWindowWidth: usize = 0x541DB8;
}
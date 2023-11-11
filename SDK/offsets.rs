/*
 * https://github.com/a2x/cs2-dumper
 * 2023-11-11 18:11:10.030385400 UTC
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod ClientDll {
    pub const dwBaseEntityModel_setModel: usize = 0x583320;
    pub const dwEntityList: usize = 0x17ADAE0;
    pub const dwForceBackward: usize = 0x16B25E0;
    pub const dwForceCrouch: usize = 0x16B28B0;
    pub const dwForceForward: usize = 0x16B2550;
    pub const dwForceJump: usize = 0x16B2820;
    pub const dwForceLeft: usize = 0x16B2670;
    pub const dwForceRight: usize = 0x16B2700;
    pub const dwGameEntitySystem_getBaseEntity: usize = 0x606930;
    pub const dwGameEntitySystem_getHighestEntityIndex: usize = 0x5F8620;
    pub const dwGameRules: usize = 0x1809720;
    pub const dwGlobalVars: usize = 0x16AE498;
    pub const dwGlowManager: usize = 0x1809748;
    pub const dwInterfaceLinkList: usize = 0x1904F68;
    pub const dwLocalPlayerController: usize = 0x17FCDB8;
    pub const dwLocalPlayerPawn: usize = 0x16B9368;
    pub const dwPlantedC4: usize = 0x1810CC8;
    pub const dwViewAngles: usize = 0x186B0F0;
    pub const dwViewMatrix: usize = 0x180C0E0;
    pub const dwViewRender: usize = 0x180C938;
}

pub mod Engine2Dll {
    pub const dwBuildNumber: usize = 0x48B524;
    pub const dwNetworkGameClient: usize = 0x48AAC0;
    pub const dwNetworkGameClient_getLocalPlayer: usize = 0xF0;
    pub const dwNetworkGameClient_maxClients: usize = 0x250;
    pub const dwNetworkGameClient_signOnState: usize = 0x240;
    pub const dwWindowHeight: usize = 0x541E1C;
    pub const dwWindowWidth: usize = 0x541E18;
}
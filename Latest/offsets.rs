#![allow(non_snake_case, non_upper_case_globals)]

pub mod client_dll {
    pub const entityList: usize = 0x178D8C8;
    pub const globalVars: usize = 0x1690CF8;
    pub const interfaceLinkList: usize = 0x1973A68;
    pub const localPlayerController: usize = 0x17DC148;
    pub const localPlayerPawn: usize = 0x187AC28;
    pub const plantedC4: usize = 0x1880AE0;
    pub const viewAngles: usize = 0x18DA5C0;
    pub const viewMatrix: usize = 0x187B710;
}

pub mod engine2_dll {
    pub const networkGameClient: usize = 0x48AAB0;
    pub const networkGameClient_maxClients: usize = 0x250;
    pub const networkGameClient_signOnState: usize = 0x240;
}
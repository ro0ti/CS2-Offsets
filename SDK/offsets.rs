#![allow(non_snake_case, non_upper_case_globals)]

pub mod client_dll {
    pub const entityList: usize = 0x178D8E8;
    pub const globalVars: usize = 0x1690D18;
    pub const interfaceLinkList: usize = 0x1973BA8;
    pub const localPlayerController: usize = 0x17DC158;
    pub const localPlayerPawn: usize = 0x187AC48;
    pub const plantedC4: usize = 0x1880DE0;
    pub const viewAngles: usize = 0x18DA700;
    pub const viewMatrix: usize = 0x187B730;
}

pub mod engine2_dll {
    pub const networkGameClient: usize = 0x48BAB0;
    pub const networkGameClient_maxClients: usize = 0x250;
    pub const networkGameClient_signOnState: usize = 0x240;
}
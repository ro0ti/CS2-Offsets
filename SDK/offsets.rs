#![allow(non_snake_case, non_upper_case_globals)]

pub mod client_dll {
    pub const entityList: usize = 0x178D8D8;
    pub const globalVars: usize = 0x1690D28;
    pub const interfaceLinkList: usize = 0x1973B98;
    pub const localPlayerController: usize = 0x17DC148;
    pub const localPlayerPawn: usize = 0x187AC38;
    pub const plantedC4: usize = 0x1880D80;
    pub const viewAngles: usize = 0x18DA6F0;
    pub const viewMatrix: usize = 0x187B720;
}

pub mod engine2_dll {
    pub const networkGameClient: usize = 0x48AAB0;
    pub const networkGameClient_maxClients: usize = 0x250;
    pub const networkGameClient_signOnState: usize = 0x240;
}
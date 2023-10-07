#![allow(non_snake_case, non_upper_case_globals)]

pub mod client_dll {
    pub const entityList: usize = 0x17888D8;
    pub const globalVars: usize = 0x168BCE8;
    pub const interfaceLinkList: usize = 0x196EC98;
    pub const localPlayerController: usize = 0x17D7158;
    pub const localPlayerPawn: usize = 0x1875C48;
    pub const plantedC4: usize = 0x187B9E0;
    pub const viewAngles: usize = 0x18D5700;
    pub const viewMatrix: usize = 0x1876730;
}

pub mod engine2_dll {
    pub const networkGameClient: usize = 0x485AB0;
    pub const networkGameClient_maxClients: usize = 0x250;
    pub const networkGameClient_signOnState: usize = 0x240;
}
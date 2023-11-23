/*
 * https://github.com/a2x/cs2-dumper
 * 2023-11-23 10:28:27.803796800 UTC
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod AnimationsystemDll {
    pub const AnimationSystemUtils_001: usize = 0x64670;
    pub const AnimationSystem_001: usize = 0x5F1D0;
}

pub mod ClientDll {
    pub const LegacyGameUI001: usize = 0x89C960;
    pub const Source2ClientUI001: usize = 0x88B670;
    pub const Source2ClientPrediction001: usize = 0x7A3990;
    pub const ClientToolsInfo_001: usize = 0x72F780;
    pub const Source2Client002: usize = 0x72F7A0;
    pub const GameClientExports001: usize = 0x72F790;
    pub const EmptyWorldService001_Client: usize = 0x48FD90;
    pub const Source2ClientConfig001: usize = 0x476CD0;
}

pub mod Engine2Dll {
    pub const SimpleEngineLoopService_001: usize = 0x1E4700;
    pub const ClientServerEngineLoopService_001: usize = 0x1DA350;
    pub const KeyValueCache001: usize = 0x1D6990;
    pub const HostStateMgr001: usize = 0x1D4190;
    pub const GameEventSystemServerV001: usize = 0x1CF830;
    pub const GameEventSystemClientV001: usize = 0x1CF820;
    pub const EngineServiceMgr001: usize = 0x1CB510;
    pub const VProfService_001: usize = 0x1B8E80;
    pub const ToolService_001: usize = 0x1B7BD0;
    pub const StatsService_001: usize = 0x1B2E50;
    pub const SplitScreenService_001: usize = 0x1B00C0;
    pub const SoundService_001: usize = 0x1AAC00;
    pub const ScreenshotService001: usize = 0x1A6AD0;
    pub const RenderService_001: usize = 0x1A3E30;
    pub const NetworkService_001: usize = 0x1A3940;
    pub const NetworkServerService_001: usize = 0x19E7F0;
    pub const NetworkP2PService_001: usize = 0x19A2E0;
    pub const NetworkClientService_001: usize = 0x193F70;
    pub const MapListService_001: usize = 0x18CF40;
    pub const InputService_001: usize = 0x17ABB0;
    pub const GameUIService_001: usize = 0x1753D0;
    pub const GameResourceServiceServerV001: usize = 0x173130;
    pub const GameResourceServiceClientV001: usize = 0x173120;
    pub const BugService001: usize = 0x16C150;
    pub const BenchmarkService001: usize = 0x16A920;
    pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x118430;
    pub const EngineGameUI001: usize = 0x1177C0;
    pub const INETSUPPORT_001: usize = 0xE6E20;
    pub const Source2EngineToServerStringTable001: usize = 0xA2410;
    pub const Source2EngineToServer001: usize = 0x8B6F0;
    pub const Source2EngineToClientStringTable001: usize = 0x841D0;
    pub const Source2EngineToClient001: usize = 0x5A990;
}

pub mod FilesystemStdioDll {
    pub const VAsyncFileSystem2_001: usize = 0x66D30;
    pub const VFileSystem017: usize = 0x66D20;
}

pub mod HostDll {
    pub const Source2Host001: usize = 0x18E10;
    pub const SinglePlayerSharedMemory001: usize = 0x18A90;
    pub const SaveRestoreDataVersion001: usize = 0x18A80;
    pub const PredictionDiffManager001: usize = 0x16E20;
    pub const HostUtils001: usize = 0x12550;
    pub const GameSystem2HostHook: usize = 0x11DA0;
    pub const GameModelInfo001: usize = 0x11CC0;
    pub const DebugDrawQueueManager001: usize = 0x11710;
}

pub mod ImemanagerDll {
    pub const IMEManager001: usize = 0xC470;
}

pub mod InputsystemDll {
    pub const InputSystemVersion001: usize = 0x28D0;
    pub const InputStackSystemVersion001: usize = 0x16F0;
}

pub mod LocalizeDll {
    pub const Localize_001: usize = 0x3830;
}

pub mod MatchmakingDll {
    pub const GameTypes001: usize = 0x50270;
    pub const MATCHFRAMEWORK_001: usize = 0x1012B0;
}

pub mod Materialsystem2Dll {
    pub const MaterialUtils_001: usize = 0x4DB90;
    pub const TextLayout_001: usize = 0x4A2E0;
    pub const PostProcessingSystem_001: usize = 0x42A60;
    pub const FontManager_001: usize = 0x37900;
    pub const VMaterialSystem2_001: usize = 0x25EC0;
}

pub mod MeshsystemDll {
    pub const MeshSystem001: usize = 0x7270;
}

pub mod NavsystemDll {
    pub const NavSystem001: usize = 0x76F0;
}

pub mod NetworksystemDll {
    pub const SerializedEntitiesVersion001: usize = 0xD1EB0;
    pub const NetworkSystemVersion001: usize = 0xBC6E0;
    pub const NetworkMessagesVersion001: usize = 0x9C6D0;
    pub const FlattenedSerializersVersion001: usize = 0x7C440;
}

pub mod PanoramaDll {
    pub const PanoramaUIEngine001: usize = 0x57EE0;
}

pub mod PanoramaTextPangoDll {
    pub const PanoramaTextServices001: usize = 0x4CBD0;
}

pub mod PanoramauiclientDll {
    pub const PanoramaUIClient001: usize = 0x12780;
}

pub mod ParticlesDll {
    pub const ParticleSystemMgr003: usize = 0x52D20;
}

pub mod PulseSystemDll {
    pub const IPulseSystem_001: usize = 0x5B30;
}

pub mod Rendersystemdx11Dll {
    pub const RenderUtils_001: usize = 0x530F0;
    pub const VRenderDeviceMgrBackdoor001: usize = 0x4A690;
    pub const RenderDeviceMgr001: usize = 0x4A680;
}

pub mod ResourcesystemDll {
    pub const ResourceSystem013: usize = 0x10650;
}

pub mod ScenefilecacheDll {
    pub const SceneFileCache002: usize = 0x68E0;
    pub const ResponseRulesCache001: usize = 0x3190;
}

pub mod ScenesystemDll {
    pub const SceneUtils_001: usize = 0x13C9E0;
    pub const SceneSystem_002: usize = 0xCAA30;
    pub const RenderingPipelines_001: usize = 0x8E8A0;
}

pub mod SchemasystemDll {
    pub const SchemaSystem_001: usize = 0xA930;
}

pub mod ServerDll {
    pub const NavGameTest001: usize = 0xA3A970;
    pub const ServerToolsInfo_001: usize = 0x83D770;
    pub const Source2GameClients001: usize = 0x83D780;
    pub const Source2GameEntities001: usize = 0x83D790;
    pub const Source2Server001: usize = 0x83D7A0;
    pub const EmptyWorldService001_Server: usize = 0x591C00;
    pub const Source2ServerConfig001: usize = 0x571B30;
    pub const EntitySubclassUtilsV001: usize = 0x2D08A0;
    pub const customnavsystem001: usize = 0x24D620;
    pub const Source2GameDirector001: usize = 0x1453E0;
}

pub mod SoundsystemDll {
    pub const SoundOpSystem001: usize = 0x1567E0;
    pub const SoundOpSystemEdit001: usize = 0x8BBC0;
    pub const VMixEditTool001: usize = 0x71740;
    pub const SoundSystem001: usize = 0x46540;
}

pub mod SteamaudioDll {
    pub const SteamAudio001: usize = 0x12EE0;
}

pub mod Steamclient64Dll {
    pub const IVALIDATE001: usize = 0x84FD80;
    pub const CLIENTENGINE_INTERFACE_VERSION005: usize = 0x84BC30;
    pub const SteamClient021: usize = 0x6460C0;
    pub const SteamClient020: usize = 0x6460B0;
    pub const SteamClient019: usize = 0x6460A0;
    pub const SteamClient018: usize = 0x646090;
    pub const SteamClient017: usize = 0x646080;
    pub const SteamClient016: usize = 0x646070;
    pub const SteamClient015: usize = 0x646060;
    pub const SteamClient014: usize = 0x646050;
    pub const SteamClient013: usize = 0x646040;
    pub const SteamClient012: usize = 0x646030;
    pub const SteamClient011: usize = 0x646020;
    pub const SteamClient010: usize = 0x646010;
    pub const SteamClient009: usize = 0x646000;
    pub const SteamClient008: usize = 0x645FF0;
    pub const SteamClient007: usize = 0x645FE0;
    pub const SteamClient006: usize = 0x645FD0;
    pub const p2pvoice002: usize = 0xD97D0;
    pub const p2pvoicesingleton002: usize = 0xD6200;
}

pub mod Tier0Dll {
    pub const VStringTokenSystem001: usize = 0x18C2B0;
    pub const TestScriptMgr001: usize = 0x13F610;
    pub const VProcessUtils002: usize = 0x12F790;
    pub const VEngineCvar007: usize = 0x61920;
}

pub mod V8SystemDll {
    pub const Source2V8System001: usize = 0x1670;
}

pub mod ValveAviDll {
    pub const VAvi001: usize = 0x2F90;
}

pub mod ValveWmfDll {
    pub const VMediaFoundation001: usize = 0x12D0;
}

pub mod Vphysics2Dll {
    pub const VPhysics2_Handle_Interface_001: usize = 0x61120;
    pub const VPhysics2_Interface_001: usize = 0x5C740;
}

pub mod VscriptDll {
    pub const VScriptManager010: usize = 0x31DA0;
}

pub mod VstdlibS64Dll {
    pub const IVALIDATE001: usize = 0x25000;
    pub const VEngineCvar002: usize = 0x5760;
}

pub mod WorldrendererDll {
    pub const WorldRendererMgr001: usize = 0x21650;
}
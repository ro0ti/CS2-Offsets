/*
 * https://github.com/a2x/cs2-dumper
 * 2023-11-11 18:11:09.522904700 UTC
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod AnimationsystemDll {
    pub const AnimationSystemUtils_001: usize = 0x64670;
    pub const AnimationSystem_001: usize = 0x5F1D0;
}

pub mod ClientDll {
    pub const LegacyGameUI001: usize = 0x89C1E0;
    pub const Source2ClientUI001: usize = 0x88AEF0;
    pub const Source2ClientPrediction001: usize = 0x7A3C80;
    pub const ClientToolsInfo_001: usize = 0x72F6E0;
    pub const Source2Client002: usize = 0x72F700;
    pub const GameClientExports001: usize = 0x72F6F0;
    pub const EmptyWorldService001_Client: usize = 0x48F1F0;
    pub const Source2ClientConfig001: usize = 0x476200;
}

pub mod Engine2Dll {
    pub const SimpleEngineLoopService_001: usize = 0x1E5110;
    pub const ClientServerEngineLoopService_001: usize = 0x1DADE0;
    pub const KeyValueCache001: usize = 0x1D7420;
    pub const HostStateMgr001: usize = 0x1D4C20;
    pub const GameEventSystemServerV001: usize = 0x1D02C0;
    pub const GameEventSystemClientV001: usize = 0x1D02B0;
    pub const EngineServiceMgr001: usize = 0x1CBFA0;
    pub const VProfService_001: usize = 0x1B9950;
    pub const ToolService_001: usize = 0x1B86A0;
    pub const StatsService_001: usize = 0x1B3920;
    pub const SplitScreenService_001: usize = 0x1B0B90;
    pub const SoundService_001: usize = 0x1AB6D0;
    pub const ScreenshotService001: usize = 0x1A75A0;
    pub const RenderService_001: usize = 0x1A4900;
    pub const NetworkService_001: usize = 0x1A4410;
    pub const NetworkServerService_001: usize = 0x19F2C0;
    pub const NetworkP2PService_001: usize = 0x19ADB0;
    pub const NetworkClientService_001: usize = 0x1949E0;
    pub const MapListService_001: usize = 0x18D9B0;
    pub const InputService_001: usize = 0x17B830;
    pub const GameUIService_001: usize = 0x176050;
    pub const GameResourceServiceServerV001: usize = 0x173DB0;
    pub const GameResourceServiceClientV001: usize = 0x173DA0;
    pub const BugService001: usize = 0x16CDD0;
    pub const BenchmarkService001: usize = 0x16B5A0;
    pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x118B20;
    pub const EngineGameUI001: usize = 0x117EB0;
    pub const INETSUPPORT_001: usize = 0xE7510;
    pub const Source2EngineToServerStringTable001: usize = 0xA23D0;
    pub const Source2EngineToServer001: usize = 0x8B710;
    pub const Source2EngineToClientStringTable001: usize = 0x841F0;
    pub const Source2EngineToClient001: usize = 0x5AC30;
}

pub mod FilesystemStdioDll {
    pub const VAsyncFileSystem2_001: usize = 0x66D30;
    pub const VFileSystem017: usize = 0x66D20;
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

pub mod NetworksystemDll {
    pub const SerializedEntitiesVersion001: usize = 0xD2830;
    pub const NetworkSystemVersion001: usize = 0xBCCF0;
    pub const NetworkMessagesVersion001: usize = 0x9CD90;
    pub const FlattenedSerializersVersion001: usize = 0x7C600;
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
    pub const IPulseSystem_001: usize = 0x5B80;
}

pub mod Rendersystemdx11Dll {
    pub const RenderUtils_001: usize = 0x52FD0;
    pub const VRenderDeviceMgrBackdoor001: usize = 0x4A4F0;
    pub const RenderDeviceMgr001: usize = 0x4A4E0;
}

pub mod ResourcesystemDll {
    pub const ResourceSystem013: usize = 0x10650;
}

pub mod ScenesystemDll {
    pub const SceneUtils_001: usize = 0x13C370;
    pub const SceneSystem_002: usize = 0xCA160;
    pub const RenderingPipelines_001: usize = 0x8E420;
}

pub mod SchemasystemDll {
    pub const SchemaSystem_001: usize = 0xA930;
}

pub mod ServerDll {
    pub const NavGameTest001: usize = 0xA37380;
    pub const ServerToolsInfo_001: usize = 0x839CB0;
    pub const Source2GameClients001: usize = 0x839CC0;
    pub const Source2GameEntities001: usize = 0x839CD0;
    pub const Source2Server001: usize = 0x839CE0;
    pub const EmptyWorldService001_Server: usize = 0x58E590;
    pub const Source2ServerConfig001: usize = 0x56E5A0;
    pub const EntitySubclassUtilsV001: usize = 0x2CDA70;
    pub const customnavsystem001: usize = 0x24A6E0;
    pub const Source2GameDirector001: usize = 0x152060;
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
    pub const IVALIDATE001: usize = 0x83A900;
    pub const CLIENTENGINE_INTERFACE_VERSION005: usize = 0x8367B0;
    pub const SteamClient020: usize = 0x631440;
    pub const SteamClient019: usize = 0x631430;
    pub const SteamClient018: usize = 0x631420;
    pub const SteamClient017: usize = 0x631410;
    pub const SteamClient016: usize = 0x631400;
    pub const SteamClient015: usize = 0x6313F0;
    pub const SteamClient014: usize = 0x6313E0;
    pub const SteamClient013: usize = 0x6313D0;
    pub const SteamClient012: usize = 0x6313C0;
    pub const SteamClient011: usize = 0x6313B0;
    pub const SteamClient010: usize = 0x6313A0;
    pub const SteamClient009: usize = 0x631390;
    pub const SteamClient008: usize = 0x631380;
    pub const SteamClient007: usize = 0x631370;
    pub const SteamClient006: usize = 0x631360;
    pub const p2pvoice002: usize = 0xD93D0;
    pub const p2pvoicesingleton002: usize = 0xD5D70;
}

pub mod Tier0Dll {
    pub const VStringTokenSystem001: usize = 0x18C100;
    pub const TestScriptMgr001: usize = 0x13F460;
    pub const VProcessUtils002: usize = 0x12F5E0;
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
    pub const IVALIDATE001: usize = 0x24FF0;
    pub const VEngineCvar002: usize = 0x5750;
}

pub mod WorldrendererDll {
    pub const WorldRendererMgr001: usize = 0x21650;
}
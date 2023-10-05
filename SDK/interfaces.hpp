#pragma once

#include <cstddef>

namespace animationsystem_dll {
    constexpr std::ptrdiff_t AnimationSystemUtils_001 = 0x64510;
    constexpr std::ptrdiff_t AnimationSystem_001 = 0x5F070;
}

namespace client_dll {
    constexpr std::ptrdiff_t LegacyGameUI001 = 0x88DF10;
    constexpr std::ptrdiff_t Source2ClientUI001 = 0x87C900;
    constexpr std::ptrdiff_t Source2ClientPrediction001 = 0x79BC60;
    constexpr std::ptrdiff_t ClientToolsInfo_001 = 0x727A90;
    constexpr std::ptrdiff_t Source2Client002 = 0x727AB0;
    constexpr std::ptrdiff_t GameClientExports001 = 0x727AA0;
    constexpr std::ptrdiff_t EmptyWorldService001_Client = 0x48AB60;
    constexpr std::ptrdiff_t Source2ClientConfig001 = 0x471EC0;
}

namespace engine2_dll {
    constexpr std::ptrdiff_t SimpleEngineLoopService_001 = 0x1E51D0;
    constexpr std::ptrdiff_t ClientServerEngineLoopService_001 = 0x1DAEA0;
    constexpr std::ptrdiff_t KeyValueCache001 = 0x1D7240;
    constexpr std::ptrdiff_t HostStateMgr001 = 0x1D4C50;
    constexpr std::ptrdiff_t GameEventSystemServerV001 = 0x1D0070;
    constexpr std::ptrdiff_t GameEventSystemClientV001 = 0x1D0060;
    constexpr std::ptrdiff_t EngineServiceMgr001 = 0x1CB980;
    constexpr std::ptrdiff_t VProfService_001 = 0x1B8F70;
    constexpr std::ptrdiff_t ToolService_001 = 0x1B7CD0;
    constexpr std::ptrdiff_t StatsService_001 = 0x1B2F50;
    constexpr std::ptrdiff_t SplitScreenService_001 = 0x1B01C0;
    constexpr std::ptrdiff_t SoundService_001 = 0x1AAD10;
    constexpr std::ptrdiff_t ScreenshotService001 = 0x1A6BE0;
    constexpr std::ptrdiff_t RenderService_001 = 0x1A3F40;
    constexpr std::ptrdiff_t NetworkService_001 = 0x1A3A50;
    constexpr std::ptrdiff_t NetworkServerService_001 = 0x19EAF0;
    constexpr std::ptrdiff_t NetworkP2PService_001 = 0x19A500;
    constexpr std::ptrdiff_t NetworkClientService_001 = 0x194130;
    constexpr std::ptrdiff_t MapListService_001 = 0x18D100;
    constexpr std::ptrdiff_t InputService_001 = 0x17AE50;
    constexpr std::ptrdiff_t GameUIService_001 = 0x175910;
    constexpr std::ptrdiff_t GameResourceServiceServerV001 = 0x173670;
    constexpr std::ptrdiff_t GameResourceServiceClientV001 = 0x173660;
    constexpr std::ptrdiff_t BugService001 = 0x16C690;
    constexpr std::ptrdiff_t BenchmarkService001 = 0x16AE60;
    constexpr std::ptrdiff_t VENGINE_GAMEUIFUNCS_VERSION005 = 0x118580;
    constexpr std::ptrdiff_t EngineGameUI001 = 0x117840;
    constexpr std::ptrdiff_t INETSUPPORT_001 = 0xE6E50;
    constexpr std::ptrdiff_t Source2EngineToServerStringTable001 = 0xA18B0;
    constexpr std::ptrdiff_t Source2EngineToServer001 = 0x8B6E0;
    constexpr std::ptrdiff_t Source2EngineToClientStringTable001 = 0x840E0;
    constexpr std::ptrdiff_t Source2EngineToClient001 = 0x5A740;
}

namespace filesystem_stdio_dll {
    constexpr std::ptrdiff_t VAsyncFileSystem2_001 = 0x66D40;
    constexpr std::ptrdiff_t VFileSystem017 = 0x66D30;
}

namespace host_dll {
    constexpr std::ptrdiff_t Source2Host001 = 0x18E50;
    constexpr std::ptrdiff_t SinglePlayerSharedMemory001 = 0x18AD0;
    constexpr std::ptrdiff_t SaveRestoreDataVersion001 = 0x18AC0;
    constexpr std::ptrdiff_t PredictionDiffManager001 = 0x16E60;
    constexpr std::ptrdiff_t HostUtils001 = 0x123F0;
    constexpr std::ptrdiff_t GameSystem2HostHook = 0x11C40;
    constexpr std::ptrdiff_t GameModelInfo001 = 0x11B60;
    constexpr std::ptrdiff_t DebugDrawQueueManager001 = 0x115B0;
}

namespace imemanager_dll {
    constexpr std::ptrdiff_t IMEManager001 = 0xC470;
}

namespace inputsystem_dll {
    constexpr std::ptrdiff_t InputSystemVersion001 = 0x28D0;
    constexpr std::ptrdiff_t InputStackSystemVersion001 = 0x16F0;
}

namespace localize_dll {
    constexpr std::ptrdiff_t Localize_001 = 0x3830;
}

namespace matchmaking_dll {
    constexpr std::ptrdiff_t GameTypes001 = 0x50110;
    constexpr std::ptrdiff_t MATCHFRAMEWORK_001 = 0x101180;
}

namespace materialsystem2_dll {
    constexpr std::ptrdiff_t MaterialUtils_001 = 0x4DDC0;
    constexpr std::ptrdiff_t TextLayout_001 = 0x4A510;
    constexpr std::ptrdiff_t PostProcessingSystem_001 = 0x42C90;
    constexpr std::ptrdiff_t FontManager_001 = 0x37A70;
    constexpr std::ptrdiff_t VMaterialSystem2_001 = 0x25FD0;
}

namespace meshsystem_dll {
    constexpr std::ptrdiff_t MeshSystem001 = 0x7110;
}

namespace navsystem_dll {
    constexpr std::ptrdiff_t NavSystem001 = 0x7590;
}

namespace networksystem_dll {
    constexpr std::ptrdiff_t SerializedEntitiesVersion001 = 0xD2A60;
    constexpr std::ptrdiff_t NetworkSystemVersion001 = 0xBCD00;
    constexpr std::ptrdiff_t NetworkMessagesVersion001 = 0x9CD30;
    constexpr std::ptrdiff_t FlattenedSerializersVersion001 = 0x7C4E0;
}

namespace panorama_dll {
    constexpr std::ptrdiff_t PanoramaUIEngine001 = 0x5E650;
}

namespace panorama_text_pango_dll {
    constexpr std::ptrdiff_t PanoramaTextServices001 = 0x4CCB0;
}

namespace panoramauiclient_dll {
    constexpr std::ptrdiff_t PanoramaUIClient001 = 0x12620;
}

namespace particles_dll {
    constexpr std::ptrdiff_t ParticleSystemMgr003 = 0x52D00;
}

namespace pulse_system_dll {
    constexpr std::ptrdiff_t IPulseSystem_001 = 0x5A20;
}

namespace rendersystemdx11_dll {
    constexpr std::ptrdiff_t RenderUtils_001 = 0x53470;
    constexpr std::ptrdiff_t VRenderDeviceMgrBackdoor001 = 0x4AAD0;
    constexpr std::ptrdiff_t RenderDeviceMgr001 = 0x4AAC0;
}

namespace resourcesystem_dll {
    constexpr std::ptrdiff_t ResourceSystem013 = 0x106B0;
}

namespace scenefilecache_dll {
    constexpr std::ptrdiff_t SceneFileCache002 = 0x68E0;
    constexpr std::ptrdiff_t ResponseRulesCache001 = 0x3190;
}

namespace scenesystem_dll {
    constexpr std::ptrdiff_t SceneUtils_001 = 0x13D620;
    constexpr std::ptrdiff_t SceneSystem_002 = 0xCAAC0;
    constexpr std::ptrdiff_t RenderingPipelines_001 = 0x8EA20;
}

namespace schemasystem_dll {
    constexpr std::ptrdiff_t SchemaSystem_001 = 0xA840;
}

namespace server_dll {
    constexpr std::ptrdiff_t NavGameTest001 = 0xA2DCD0;
    constexpr std::ptrdiff_t ServerToolsInfo_001 = 0x8323B0;
    constexpr std::ptrdiff_t Source2GameClients001 = 0x8323C0;
    constexpr std::ptrdiff_t Source2GameEntities001 = 0x8323D0;
    constexpr std::ptrdiff_t Source2Server001 = 0x8323E0;
    constexpr std::ptrdiff_t EmptyWorldService001_Server = 0x586F20;
    constexpr std::ptrdiff_t Source2ServerConfig001 = 0x566EC0;
    constexpr std::ptrdiff_t EntitySubclassUtilsV001 = 0x2C8200;
    constexpr std::ptrdiff_t customnavsystem001 = 0x244CE0;
    constexpr std::ptrdiff_t Source2GameDirector001 = 0x13E8A0;
}

namespace soundsystem_dll {
    constexpr std::ptrdiff_t SoundOpSystem001 = 0x1560D0;
    constexpr std::ptrdiff_t SoundOpSystemEdit001 = 0x8B4B0;
    constexpr std::ptrdiff_t VMixEditTool001 = 0x71710;
    constexpr std::ptrdiff_t SoundSystem001 = 0x46510;
}

namespace steamaudio_dll {
    constexpr std::ptrdiff_t SteamAudio001 = 0x12D90;
}

namespace steamclient64_dll {
    constexpr std::ptrdiff_t IVALIDATE001 = 0x833640;
    constexpr std::ptrdiff_t CLIENTENGINE_INTERFACE_VERSION005 = 0x82F4C0;
    constexpr std::ptrdiff_t SteamClient020 = 0x62CAF0;
    constexpr std::ptrdiff_t SteamClient019 = 0x62CAE0;
    constexpr std::ptrdiff_t SteamClient018 = 0x62CAD0;
    constexpr std::ptrdiff_t SteamClient017 = 0x62CAC0;
    constexpr std::ptrdiff_t SteamClient016 = 0x62CAB0;
    constexpr std::ptrdiff_t SteamClient015 = 0x62CAA0;
    constexpr std::ptrdiff_t SteamClient014 = 0x62CA90;
    constexpr std::ptrdiff_t SteamClient013 = 0x62CA80;
    constexpr std::ptrdiff_t SteamClient012 = 0x62CA70;
    constexpr std::ptrdiff_t SteamClient011 = 0x62CA60;
    constexpr std::ptrdiff_t SteamClient010 = 0x62CA50;
    constexpr std::ptrdiff_t SteamClient009 = 0x62CA40;
    constexpr std::ptrdiff_t SteamClient008 = 0x62CA30;
    constexpr std::ptrdiff_t SteamClient007 = 0x62CA20;
    constexpr std::ptrdiff_t SteamClient006 = 0x62CA10;
    constexpr std::ptrdiff_t p2pvoice002 = 0xD8EA0;
    constexpr std::ptrdiff_t p2pvoicesingleton002 = 0xD5840;
}

namespace tier0_dll {
    constexpr std::ptrdiff_t VStringTokenSystem001 = 0x18D680;
    constexpr std::ptrdiff_t TestScriptMgr001 = 0x13F7B0;
    constexpr std::ptrdiff_t VProcessUtils002 = 0x12F7C0;
    constexpr std::ptrdiff_t VEngineCvar007 = 0x61640;
}

namespace v8system_dll {
    constexpr std::ptrdiff_t Source2V8System001 = 0x1670;
}

namespace valve_avi_dll {
    constexpr std::ptrdiff_t VAvi001 = 0x2F90;
}

namespace valve_wmf_dll {
    constexpr std::ptrdiff_t VMediaFoundation001 = 0x12D0;
}

namespace vphysics2_dll {
    constexpr std::ptrdiff_t VPhysics2_Handle_Interface_001 = 0x5F8F0;
    constexpr std::ptrdiff_t VPhysics2_Interface_001 = 0x5B690;
}

namespace vscript_dll {
    constexpr std::ptrdiff_t VScriptManager010 = 0x31C40;
}

namespace vstdlib_s64_dll {
    constexpr std::ptrdiff_t IVALIDATE001 = 0x24FF0;
    constexpr std::ptrdiff_t VEngineCvar002 = 0x5750;
}

namespace worldrenderer_dll {
    constexpr std::ptrdiff_t WorldRendererMgr001 = 0x21320;
}
# CS2 Offsets
I will try to get updated after every update.

# Quick Offsets
```
public static class client_dll {
    public const nint entityList = 0x178B898;
    public const nint globalVars = 0x168ECE8;
    public const nint interfaceLinkList = 0x1971A58;
    public const nint localPlayerController = 0x17DA118;
    public const nint localPlayerPawn = 0x1878C08;
    public const nint plantedC4 = 0x187E9A0;
    public const nint viewAngles = 0x18D85B0;
    public const nint viewMatrix = 0x18796F0;
}

public static class engine2_dll {
    public const nint networkGameClient = 0x48AAB0;
    public const nint networkGameClient_maxClients = 0x250;
    public const nint networkGameClient_signOnState = 0x240;
}
```

# Quick config.json
```
{
    "dwEntityList": 24688792,
    "dwLocalPlayer": 25010456,
    "dwPawnHealth": 2056,
    "dwPlayerPawn": 1500,
    "dwSanitizedName": 1824,
    "dwViewMatrix": 25663216,
    "m_aimPunchAngle": 5892,
    "m_bSpotted": 8,
    "m_bSpottedByMask": 12,
    "m_iShotsFired": 5124,
    "m_iTeamNum": 959,
    "m_vecOrigin": 4628,
    "m_vecViewOffset": 3144
}
```
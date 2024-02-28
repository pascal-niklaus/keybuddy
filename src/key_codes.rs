pub struct RawCodes {}

impl RawCodes {
    pub const KeyLeftShift : u16 = 42;
    pub const KeyRightShift : u16 = 54;
    pub const KeyRightCtrl : u16 = 97;
    pub const KeyLeftCtrl : u16 = 29;
    pub const KeyLeftAlt : u16 = 56;
    pub const KeyRightAlt : u16 = 100;
    pub const KeyLeftMeta : u16 = 125;
    pub const KeyRightMeta : u16 = 126;
    pub const KeyCompose : u16 = 127;
}


/// Raw key codes obtained by reading /dev/input
#[repr(u16)]
pub enum RawKeyCode {
    KeyEsc = 1,
    Key1 = 2,
    Key2 = 3,
    Key3 = 4,
    Key4 = 5,
    Key5 = 6,
    Key6 = 7,
    Key7 = 8,
    Key8 = 9,
    Key9 = 10,
    Key0 = 11,
    KeyMinus = 12,
    KeyEqual = 13,
    KeyBackspace = 14,
    KeyTab = 15,
    KeyQ = 16,
    KeyW = 17,
    KeyE = 18,
    KeyR = 19,
    KeyT = 20,
    KeyY = 21,
    KeyU = 22,
    KeyI = 23,
    KeyO = 24,
    KeyP = 25,
    KeyLeftBrace = 26,
    KeyRightBrace = 27,
    KeyEnter = 28,
    KeyLeftCtrl = 29,
    KeyA = 30,
    KeyS = 31,
    KeyD = 32,
    KeyF = 33,
    KeyG = 34,
    KeyH = 35,
    KeyJ = 36,
    KeyK = 37,
    KeyL = 38,
    KeySemicolon = 39,
    KeyApostrophe = 40,
    KeyGrave = 41,
    KeyLeftShift = 42,
    KeyBackslash = 43,
    KeyZ = 44,
    KeyX = 45,
    KeyC = 46,
    KeyV = 47,
    KeyB = 48,
    KeyN = 49,
    KeyM = 50,
    KeyComma = 51,
    KeyDot = 52,
    KeySlash = 53,
    KeyRightShift = 54,
    KeyKPAsterisk = 55,
    KeyLeftAlt = 56,
    KeySpace = 57,
    KeyCapslock = 58,
    KeyF1 = 59,
    KeyF2 = 60,
    KeyF3 = 61,
    KeyF4 = 62,
    KeyF5 = 63,
    KeyF6 = 64,
    KeyF7 = 65,
    KeyF8 = 66,
    KeyF9 = 67,
    KeyF10 = 68,
    KeyNumlock = 69,
    KeyScrolllock = 70,
    KeyKP7 = 71,
    KeyKP8 = 72,
    KeyKP9 = 73,
    KeyKPminus = 74,
    KeyKP4 = 75,
    KeyKP5 = 76,
    KeyKP6 = 77,
    KeyKPplus = 78,
    KeyKP1 = 79,
    KeyKP2 = 80,
    KeyKP3 = 81,
    KeyKP0 = 82,
    KeyKPdot = 83,
    KeyZenkakuhankaku = 85,
    Key102ND = 86,
    KeyF11 = 87,
    KeyF12 = 88,
    KeyRo = 89,
    KeyKatakana = 90,
    KeyHiragana = 91,
    KeyHenkan = 92,
    KeyKatakanahiragana = 93,
    KeyMuhenkan = 94,
    KeyKPjpcomma = 95,
    KeyKPenter = 96,
    KeyRightCtrl = 97,
    KeyKPslash = 98,
    KeySysrq = 99,
    KeyRightAlt = 100,
    KeyLinefeed = 101,
    KeyHome = 102,
    KeyUp = 103,
    KeyPageup = 104,
    KeyLeft = 105,
    KeyRight = 106,
    KeyEnd = 107,
    KeyDown = 108,
    KeyPagedown = 109,
    KeyInsert = 110,
    KeyDelete = 111,
    KeyMacro = 112,
    KeyMute = 113,
    KeyVolumedown = 114,
    KeyVolumeup = 115,
    KeyPower = 116,             /* SC System Power Down */
    KeyKPequal = 117,
    KeyKPplusminus = 118,
    KeyPause = 119,
    KeyScale = 120,             /* AL Compiz Scale (Expose) */
    KeyKPcomma = 121,
    KeyHangeul = 122,
    KeyHanja = 123,
    KeyYen = 124,
    KeyLeftMeta = 125,
    KeyRightMeta = 126,
    KeyCompose = 127,
    KeyStop = 128,              /* AC Stop */
    KeyAgain = 129,
    KeyProps = 130,             /* AC Properties */
    KeyUndo = 131,              /* AC Undo */
    KeyFront = 132,
    KeyCopy = 133,              /* AC Copy */
    KeyOpen = 134,              /* AC Open */
    KeyPaste = 135,             /* AC Paste */
    KeyFind = 136,              /* AC Search */
    KeyCut = 137,               /* AC Cut */
    KeyHelp = 138,              /* AL Integrated Help Center */
    KeyMenu = 139,              /* Menu (show menu) */
    KeyCalc = 140,              /* AL Calculator */
    KeySetup = 141,
    KeySleep = 142,             /* SC System Sleep */
    KeyWakeup = 143,            /* System Wake Up */
    KeyFile = 144,              /* AL Local Machine Browser */
    KeySendfile = 145,
    KeyDeletefile = 146,
    KeyXfer = 147,
    KeyProg1 = 148,
    KeyProg2 = 149,
    KeyWww = 150,               /* AL Internet Browser */
    KeyMsdos = 151,
    KeyScreenlock = 152,
    KeyRotateDisplay = 153,    /* Display orientation for e.g. tablets */
    KeyCyclewindows = 154,
    KeyMail = 155,
    KeyBookmarks = 156,         /* AC Bookmarks */
    KeyComputer = 157,
    KeyBack = 158,              /* AC Back */
    KeyForward = 159,           /* AC Forward */
    KeyClosecd = 160,
    KeyEjectcd = 161,
    KeyEjectclosecd = 162,
    KeyNextsong = 163,
    KeyPlaypause = 164,
    KeyPrevioussong = 165,
    KeyStopcd = 166,
    KeyRecord = 167,
    KeyRewind = 168,
    KeyPhone = 169,             /* Media Select Telephone */
    KeyIso = 170,
    KeyConfig = 171,            /* AL Consumer Control Configuration */
    KeyHomepage = 172,          /* AC Home */
    KeyRefresh = 173,           /* AC Refresh */
    KeyExit = 174,              /* AC Exit */
    KeyMove = 175,
    KeyEdit = 176,
    KeyScrollup = 177,
    KeyScrolldown = 178,
    KeyKPleftparen = 179,
    KeyKPrightparen = 180,
    KeyNew = 181,               /* AC New */
    KeyRedo = 182,              /* AC Redo/Repeat */
    KeyF13 = 183,
    KeyF14 = 184,
    KeyF15 = 185,
    KeyF16 = 186,
    KeyF17 = 187,
    KeyF18 = 188,
    KeyF19 = 189,
    KeyF20 = 190,
    KeyF21 = 191,
    KeyF22 = 192,
    KeyF23 = 193,
    KeyF24 = 194,
    KeyPlaycd = 200,
    KeyPausecd = 201,
    KeyProg3 = 202,
    KeyProg4 = 203,
    KeyAllApplications = 204,  /* AC Desktop Show All Applications */
    KeySuspend = 205,
    KeyClose = 206,             /* AC Close */
    KeyPlay = 207,
    KeyFastforward = 208,
    KeyBassboost = 209,
    KeyPrint = 210,             /* AC Print */
    KeyHp = 211,
    KeyCamera = 212,
    KeySound = 213,
    KeyQuestion = 214,
    KeyEmail = 215,
    KeyChat = 216,
    KeySearch = 217,
    KeyConnect = 218,
    KeyFinance = 219,           /* AL Checkbook/Finance */
    KeySport = 220,
    KeyShop = 221,
    KeyAlterase = 222,
    KeyCancel = 223,            /* AC Cancel */
    KeyBrightnessdown = 224,
    KeyBrightnessup = 225,
    KeyMedia = 226,
    KeySwitchvideomode = 227,
    KeyKbdillumtoggle = 228,
    KeyKbdillumdown = 229,
    KeyKbdillumup = 230,
    KeySend = 231,              /* AC Send */
    KeyReply = 232,             /* AC Reply */
    KeyForwardmail = 233,       /* AC Forward Msg */
    KeySave = 234,              /* AC Save */
    KeyDocuments = 235,
    KeyBattery = 236,
    KeyBluetooth = 237,
    KeyWlan = 238,
    KeyUwb = 239,
    KeyUnknown = 240,
    KeyVideoNext = 241,        /* drive next video source */
    KeyVideoPrev = 242,        /* drive previous video source */
    KeyBrightnessCycle = 243,  /* brightness up, after max is min */
    KeyBrightnessAuto = 244,   /* Set Auto Brightness */
    KeyDisplayOff = 245,       /* display device to off state */
    KeyWwan = 246,              /* Wireless WAN (LTE, UMTS, GSM, etc.) */
    KeyRfkill = 247,            /* Key that controls all radios */
    KeyMicmute = 248,           /* Mute / unmute the microphone */
}

/*

#define KEY_OK                  0x160
#define KEY_SELECT              0x161
#define KEY_GOTO                0x162
#define KEY_CLEAR               0x163
#define KEY_POWER2              0x164
#define KEY_OPTION              0x165
#define KEY_INFO                0x166   /* AL OEM Features/Tips/Tutorial */
#define KEY_TIME                0x167
#define KEY_VENDOR              0x168
#define KEY_ARCHIVE             0x169
#define KEY_PROGRAM             0x16a   /* Media Select Program Guide */
#define KEY_CHANNEL             0x16b
#define KEY_FAVORITES           0x16c
#define KEY_EPG                 0x16d
#define KEY_PVR                 0x16e   /* Media Select Home */
#define KEY_MHP                 0x16f
#define KEY_LANGUAGE            0x170
#define KEY_TITLE               0x171
#define KEY_SUBTITLE            0x172
#define KEY_ANGLE               0x173
#define KEY_FULL_SCREEN         0x174   /* AC View Toggle */
#define KEY_ZOOM                KEY_FULL_SCREEN
#define KEY_MODE                0x175
#define KEY_KEYBOARD            0x176
#define KEY_ASPECT_RATIO        0x177   /* HUTRR37: Aspect */
#define KEY_SCREEN              KEY_ASPECT_RATIO
#define KEY_PC                  0x178   /* Media Select Computer */
#define KEY_TV                  0x179   /* Media Select TV */
#define KEY_TV2                 0x17a   /* Media Select Cable */
#define KEY_VCR                 0x17b   /* Media Select VCR */
#define KEY_VCR2                0x17c   /* VCR Plus */
#define KEY_SAT                 0x17d   /* Media Select Satellite */
#define KEY_SAT2                0x17e
#define KEY_CD                  0x17f   /* Media Select CD */
#define KEY_TAPE                0x180   /* Media Select Tape */
#define KEY_RADIO               0x181
#define KEY_TUNER               0x182   /* Media Select Tuner */
#define KEY_PLAYER              0x183
#define KEY_TEXT                0x184
#define KEY_DVD                 0x185   /* Media Select DVD */
#define KEY_AUX                 0x186
#define KEY_MP3                 0x187
#define KEY_AUDIO               0x188   /* AL Audio Browser */
#define KEY_VIDEO               0x189   /* AL Movie Browser */
#define KEY_DIRECTORY           0x18a
#define KEY_LIST                0x18b
#define KEY_MEMO                0x18c   /* Media Select Messages */
#define KEY_CALENDAR            0x18d
#define KEY_RED                 0x18e
#define KEY_GREEN               0x18f
#define KEY_YELLOW              0x190
#define KEY_BLUE                0x191
#define KEY_CHANNELUP           0x192   /* Channel Increment */
#define KEY_CHANNELDOWN         0x193   /* Channel Decrement */
#define KEY_FIRST               0x194
#define KEY_LAST                0x195   /* Recall Last */
#define KEY_AB                  0x196
#define KEY_NEXT                0x197
#define KEY_RESTART             0x198
#define KEY_SLOW                0x199
#define KEY_SHUFFLE             0x19a
#define KEY_BREAK               0x19b
#define KEY_PREVIOUS            0x19c
#define KEY_DIGITS              0x19d
#define KEY_TEEN                0x19e
#define KEY_TWEN                0x19f
#define KEY_VIDEOPHONE          0x1a0   /* Media Select Video Phone */
#define KEY_GAMES               0x1a1   /* Media Select Games */
#define KEY_ZOOMIN              0x1a2   /* AC Zoom In */
#define KEY_ZOOMOUT             0x1a3   /* AC Zoom Out */
#define KEY_ZOOMRESET           0x1a4   /* AC Zoom */
#define KEY_WORDPROCESSOR       0x1a5   /* AL Word Processor */
#define KEY_EDITOR              0x1a6   /* AL Text Editor */
#define KEY_SPREADSHEET         0x1a7   /* AL Spreadsheet */
#define KEY_GRAPHICSEDITOR      0x1a8   /* AL Graphics Editor */
#define KEY_PRESENTATION        0x1a9   /* AL Presentation App */
#define KEY_DATABASE            0x1aa   /* AL Database App */
#define KEY_NEWS                0x1ab   /* AL Newsreader */
#define KEY_VOICEMAIL           0x1ac   /* AL Voicemail */
#define KEY_ADDRESSBOOK         0x1ad   /* AL Contacts/Address Book */
#define KEY_MESSENGER           0x1ae   /* AL Instant Messaging */
#define KEY_DISPLAYTOGGLE       0x1af   /* Turn display (LCD) on and off */
#define KEY_BRIGHTNESS_TOGGLE   KEY_DISPLAYTOGGLE
#define KEY_SPELLCHECK          0x1b0   /* AL Spell Check */
#define KEY_LOGOFF              0x1b1   /* AL Logoff */

#define KEY_DOLLAR              0x1b2
#define KEY_EURO                0x1b3

#define KEY_FRAMEBACK           0x1b4   /* Consumer - transport controls */
#define KEY_FRAMEFORWARD        0x1b5
#define KEY_CONTEXT_MENU        0x1b6   /* GenDesc - system context menu */
#define KEY_MEDIA_REPEAT        0x1b7   /* Consumer - transport control */
#define KEY_10CHANNELSUP        0x1b8   /* 10 channels up (10+) */
#define KEY_10CHANNELSDOWN      0x1b9   /* 10 channels down (10-) */
#define KEY_IMAGES              0x1ba   /* AL Image Browser */
#define KEY_NOTIFICATION_CENTER 0x1bc   /* Show/hide the notification center */
#define KEY_PICKUP_PHONE        0x1bd   /* Answer incoming call */
#define KEY_HANGUP_PHONE        0x1be   /* Decline incoming call */

#define KEY_DEL_EOL             0x1c0
#define KEY_DEL_EOS             0x1c1
#define KEY_INS_LINE            0x1c2
#define KEY_DEL_LINE            0x1c3

#define KEY_FN                  0x1d0
#define KEY_FN_ESC              0x1d1
#define KEY_FN_F1               0x1d2
#define KEY_FN_F2               0x1d3
#define KEY_FN_F3               0x1d4
#define KEY_FN_F4               0x1d5
#define KEY_FN_F5               0x1d6
#define KEY_FN_F6               0x1d7
#define KEY_FN_F7               0x1d8
#define KEY_FN_F8               0x1d9
#define KEY_FN_F9               0x1da
#define KEY_FN_F10              0x1db
#define KEY_FN_F11              0x1dc
#define KEY_FN_F12              0x1dd
#define KEY_FN_1                0x1de
#define KEY_FN_2                0x1df
#define KEY_FN_D                0x1e0
#define KEY_FN_E                0x1e1
#define KEY_FN_F                0x1e2
#define KEY_FN_S                0x1e3
#define KEY_FN_B                0x1e4
#define KEY_FN_RIGHT_SHIFT      0x1e5

#define KEY_BRL_DOT1            0x1f1
#define KEY_BRL_DOT2            0x1f2
#define KEY_BRL_DOT3            0x1f3
#define KEY_BRL_DOT4            0x1f4
#define KEY_BRL_DOT5            0x1f5
#define KEY_BRL_DOT6            0x1f6
#define KEY_BRL_DOT7            0x1f7
#define KEY_BRL_DOT8            0x1f8
#define KEY_BRL_DOT9            0x1f9
#define KEY_BRL_DOT10           0x1fa

#define KEY_NUMERIC_0           0x200   /* used by phones, remote controls, */
#define KEY_NUMERIC_1           0x201   /* and other keypads */
#define KEY_NUMERIC_2           0x202
#define KEY_NUMERIC_3           0x203
#define KEY_NUMERIC_4           0x204
#define KEY_NUMERIC_5           0x205
#define KEY_NUMERIC_6           0x206
#define KEY_NUMERIC_7           0x207
#define KEY_NUMERIC_8           0x208
#define KEY_NUMERIC_9           0x209
#define KEY_NUMERIC_STAR        0x20a
#define KEY_NUMERIC_POUND       0x20b
#define KEY_NUMERIC_A           0x20c   /* Phone key A - HUT Telephony 0xb9 */
#define KEY_NUMERIC_B           0x20d
#define KEY_NUMERIC_C           0x20e
#define KEY_NUMERIC_D           0x20f

#define KEY_CAMERA_FOCUS        0x210
#define KEY_WPS_BUTTON          0x211   /* WiFi Protected Setup key */

#define KEY_TOUCHPAD_TOGGLE     0x212   /* Request switch touchpad on or off */
#define KEY_TOUCHPAD_ON         0x213
#define KEY_TOUCHPAD_OFF        0x214

#define KEY_CAMERA_ZOOMIN       0x215
#define KEY_CAMERA_ZOOMOUT      0x216
#define KEY_CAMERA_UP           0x217
#define KEY_CAMERA_DOWN         0x218
#define KEY_CAMERA_LEFT         0x219
#define KEY_CAMERA_RIGHT        0x21a

#define KEY_ATTENDANT_ON        0x21b
#define KEY_ATTENDANT_OFF       0x21c
#define KEY_ATTENDANT_TOGGLE    0x21d   /* Attendant call on or off */
#define KEY_LIGHTS_TOGGLE       0x21e   /* Reading light on or off */

#define BTN_DPAD_UP             0x220
#define BTN_DPAD_DOWN           0x221
#define BTN_DPAD_LEFT           0x222
#define BTN_DPAD_RIGHT          0x223

#define KEY_ALS_TOGGLE          0x230   /* Ambient light sensor */
#define KEY_ROTATE_LOCK_TOGGLE  0x231   /* Display rotation lock */

#define KEY_BUTTONCONFIG                0x240   /* AL Button Configuration */
#define KEY_TASKMANAGER         0x241   /* AL Task/Project Manager */
#define KEY_JOURNAL             0x242   /* AL Log/Journal/Timecard */
#define KEY_CONTROLPANEL                0x243   /* AL Control Panel */
#define KEY_APPSELECT           0x244   /* AL Select Task/Application */
#define KEY_SCREENSAVER         0x245   /* AL Screen Saver */
#define KEY_VOICECOMMAND                0x246   /* Listening Voice Command */
#define KEY_ASSISTANT           0x247   /* AL Context-aware desktop assistant */
#define KEY_KBD_LAYOUT_NEXT     0x248   /* AC Next Keyboard Layout Select */
#define KEY_EMOJI_PICKER        0x249   /* Show/hide emoji picker (HUTRR101) */
#define KEY_DICTATE             0x24a   /* Start or Stop Voice Dictation Session (HUTRR99) */

#define KEY_BRIGHTNESS_MIN              0x250   /* Set Brightness to Minimum */
#define KEY_BRIGHTNESS_MAX              0x251   /* Set Brightness to Maximum */

#define KEY_KBDINPUTASSIST_PREV         0x260
#define KEY_KBDINPUTASSIST_NEXT         0x261
#define KEY_KBDINPUTASSIST_PREVGROUP            0x262
#define KEY_KBDINPUTASSIST_NEXTGROUP            0x263
#define KEY_KBDINPUTASSIST_ACCEPT               0x264
#define KEY_KBDINPUTASSIST_CANCEL               0x265
*/

# BC Messages

---

This is an attempt to document the BC messages. It is subject to change
and some aspects of it may not be correct. Please feel free to submit
a PR to improve it.

Message have zero to two payloads.

- The first payload is after the header and before the payload offset
  - This payload always contains Extension xml and so is called `Extension`
    in this doc
- The second payload is after the payload offset

  - This is either Body xml or binary data.
  - If it is binary the extension xml will contain the `<binary>1</binary>`
    tag
  - This is called `Payload` in this doc

- 1: Login Legacy

  - Client

    - Header

    | magic       | message id  | message length | encryption offset | encrypt | unknown | message class |
    | ----------- | ----------- | -------------- | ----------------- | ------- | ------- | ------------- |
    | f0 de bc 0a | 01 00 00 00 | 2c 07 00 00    | 00 00 00 01       | 01      | dc      | 14 65         |

    - Payload

      Body is hash of user 32 bytes and password 32 bytes and then a lot of zero pads

      ```hex
      MD5USERNAME0MD5PASSWORD00000000000000000000000000000000000000000
      0000000000000000000000000000000000000000000000000000000000000000
      .......
      ```

  - Camera

    - Header

    | magic       | message id  | message length | encryption offset | encrypt | unknown | message class |
    | ----------- | ----------- | -------------- | ----------------- | ------- | ------- | ------------- |
    | f0 de bc 0a | 01 00 00 00 | 91 00 00 00    | 00 00 00 01       | 01      | dd      | 14 66         |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Encryption version="1.1">
    <type>md5</type>
    <nonce>13BCECE33DA453DB</nonce>
    </Encryption>
    </body>
    ```

    - **Notes:** Sends back a NONCE used for the modern login message. This is
      effectively an upgrade request to use the modern xml style over legacy.
      Legacy cameras respond with status code `c8 00`, message class `00 00` and a basic camera description payload.
      The legacy protocol beyond this point is not documented and not implemented in Neolink.

- 1: Login Modern

  - Client

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | 01 00 00 00 | 28 01 00 00    | 00 00 00 01       | 00 00       | 14 64         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <LoginUser version="1.1">
    <userName>...</userName> <!-- Hash of username with nonce -->
    <password>...</password> <!-- Hash of password with nonce -->
    <userVer>1</userVer>
    </LoginUser>
    <LoginNet version="1.1">
    <type>LAN</type>
    <udpPort>0</udpPort>
    </LoginNet>
    </body>
    ```

  - Camera

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | 01 00 00 00 | 2e 06 00 00    | 00 00 00 01       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <DeviceInfo version="1.1">
    <firmVersion>00000000000000</firmVersion>
    <IOInputPortNum>0</IOInputPortNum>
    <IOOutputPortNum>0</IOOutputPortNum>
    <diskNum>0</diskNum>
    <type>wifi_solo_ipc</type>
    <channelNum>1</channelNum>
    <audioNum>1</audioNum>
    <ipChannel>0</ipChannel>
    <analogChnNum>1</analogChnNum>
    <resolution>
    <resolutionName>2304*1296</resolutionName>
    <width>2304</width>
    <height>1296</height>
    </resolution>
    <language>English</language>
    <sdCard>1</sdCard>
    <ptzMode>pt</ptzMode>
    <typeInfo>IPC</typeInfo>
    <softVer>33555019</softVer>
    <hardVer>0</hardVer>
    <panelVer>0</panelVer>
    <hdChannel1>0</hdChannel1>
    <hdChannel2>0</hdChannel2>
    <hdChannel3>0</hdChannel3>
    <hdChannel4>0</hdChannel4>
    <norm>NTSC</norm>
    <osdFormat>DMY</osdFormat>
    <B485>0</B485>
    <supportAutoUpdate>0</supportAutoUpdate>
    <userVer>1</userVer>
    </DeviceInfo>
    <StreamInfoList version="1.1">
    <StreamInfo>
    <channelBits>1</channelBits>
    <encodeTable>
    <type>mainStream</type>
    <resolution>
    <width>2304</width>
    <height>1296</height>
    </resolution>
    <defaultFramerate>15</defaultFramerate>
    <defaultBitrate>2560</defaultBitrate>
    <framerateTable>15,12,10,8,6,4,2</framerateTable>
    <bitrateTable>1024,1536,2048,2560,3072</bitrateTable>
    </encodeTable>
    <encodeTable>
    <type>subStream</type>
    <resolution>
    <width>896</width>
    <height>512</height>
    </resolution>
    <defaultFramerate>15</defaultFramerate>
    <defaultBitrate>512</defaultBitrate>
    <framerateTable>15,12,10,8,6,4,2</framerateTable>
    <bitrateTable>128,256,384,512,768,1024</bitrateTable>
    </encodeTable>
    </StreamInfo>
    </StreamInfoList>
    </body>
    ```

- 2: logout

  - Client

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | 02 00 00 00 | af 00 00 00    | 00 00 00 09       | 00 00       | 14 64         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <LoginUser version="1.1">
    <userName>PlainTextUsername</userName>
    <password>PlainTextPASSWORD</password>
    <userVer>1</userVer>
    </LoginUser>
    </body>
    ```

- 3: Stream

  - Client

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | 03 00 00 00 | aa 00 00 00    | 00 00 00 09       | 00 00       | 14 64         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Preview version="1.1">
    <channelId>0</channelId>
    <handle>0</handle>
    <streamType>mainStream</streamType>
    </Preview>
    </body>
    ```

    - **Notes:** This requests the camera to send this stream

  - Camera

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | 03 00 00 00 | 8a 00 00 00    | 00 00 00 09       | c8 00       | 00 00         | 6a 00 00 00    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <binaryData>1</binaryData>
    </Extension>
    ```

    - Payload

    ```hex
    31303032200000000009000010050000000F780A06122422780A061224220000
    ```

    - **Notes:** Camera then send the stream as a binary payload in all
      following messages of id 3

  - Camera Stream Binary

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | 03 00 00 00 | e8 5e 00 00    | 00 00 00 09       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

      Body is binary. This binary represents an embedded stream which should
      is detailed in [mediapacket.md](mediapacket.md).

- 4: `<Preview>` (stop)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 04 | 00 00 00 86    | 2b 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Preview version="1.1">
    <channelId>0</channelId>
    <handle>0</handle>
    </Preview>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 04 | 00 00 00 00    | 2b 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 10: `<TalkAbility>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 0a | 00 00 00 68    | 0b 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 0a | 00 00 01 f7    | 0b 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <TalkAbility version="1.1">
    <duplexList>
    <duplex>FDX</duplex>
    </duplexList>
    <audioStreamModeList>
    <audioStreamMode>followVideoStream</audioStreamMode>
    </audioStreamModeList>
    <audioConfigList>
    <audioConfig>
    <priority>0</priority>
    <audioType>adpcm</audioType>
    <sampleRate>16000</sampleRate>
    <samplePrecision>16</samplePrecision>
    <lengthPerEncoder>1024</lengthPerEncoder>
    <soundTrack>mono</soundTrack>
    </audioConfig>
    </audioConfigList>
    </TalkAbility>
    </body>
    ```

- 18: `<PtzControl>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 12 | 00 00 00 a4    | 1e 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <PtzControl version="1.1">
    <channelId>0</channelId>
    <speed>32</speed>
    <command>right</command>
    </PtzControl>
    </body>
    ```

    - **Notes** : The known movement commands are `"left"`, `"right"`, `"up"`, `"down"`, `"leftUp"`,
      `"leftDown"`, `"rightUp"`, `"rightDown"` and `"stop"` although the diagonal movement
      does not seem to work.

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 12 | 00 00 00 00    | 1e 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 19: `<PtzPreset>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 13 | 00 00 01 44    | 1e 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <PtzPreset version="1.1">
    <channelId>0</channelId>
    <presetList>
    <preset>
    <id>0</id>
    <command>setPos</command>
    <name>Test</name>
    </preset>
    </presetList>
    </PtzPreset>
    </body>
    ```

    - **Notes** : The known values for command are `"setPos"` and `"toPos"`

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 13 | 00 00 00 00    | 1e 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 23: `Reboot`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Binary Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | ------------- |
      | 0a bc de f0 | 00 00 00 17 | 00 00 00 00    | 00 00 00 00       | 00 00       | 64 14         | 00 00 00 00   |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Binary Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | ------------- |
      | 0a bc de f0 | 00 00 00 17 | 00 00 00 00    | 00 00 00 00       | c8 00       | 64 14         | 00 00 00 00   |

- 25: `<VideoInput>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 19 | 00 00 05 c2    | 64 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <VideoInput version="1.1">
    <channelId>0</channelId>
    <bright>128</bright>
    <contrast>128</contrast>
    <saturation>128</saturation>
    <hue>128</hue>
    <sharpen>166</sharpen>
    </VideoInput>
    <InputAdvanceCfg version="1.1">
    <channelId>0</channelId>
    <digitalChannel>1</digitalChannel>
    <PowerLineFrequency>
    <mode>50hz</mode>
    <enable>0</enable>
    </PowerLineFrequency>
    <Exposure>
    <mode>auto</mode>
    <Gainctl>
    <defMin>1</defMin>
    <defMax>100</defMax>
    <curMin>1</curMin>
    <curMax>62</curMax>
    </Gainctl>
    <Shutterctl>
    <defMin>0</defMin>
    <defMax>125</defMax>
    <curMin>0</curMin>
    <curMax>125</curMax>
    </Shutterctl>
    <shutterLevel>1/30</shutterLevel>
    <gainLevel>50</gainLevel>
    </Exposure>
    <Scene>
    <mode>auto</mode>
    <Redgain>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </Redgain>
    <Bluegain>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </Bluegain>
    </Scene>
    <DayNight>
    <mode>auto</mode>
    <IrcutMode>ir</IrcutMode>
    <Threshold>medium</Threshold>
    </DayNight>
    <BLC>
    <enable>0</enable>
    <mode>backLight</mode>
    <dynamicrange>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </dynamicrange>
    <backlight>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </backlight>
    </BLC>
    <mirror>0</mirror>
    <flip>0</flip>
    <Iris>
    <enable>0</enable>
    <state>success</state>
    <focusAutoiris>0</focusAutoiris>
    </Iris>
    <nr3d>
    <value>high</value>
    <enable>1</enable>
    </nr3d>
    </InputAdvanceCfg>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 19 | 00 00 00 00    | 64 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 26: `<VideoInput>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 1a | 00 00 00 68    | 2d 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 1a | 00 00 05 7c    | 2d 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <VideoInput version="1.1">
    <channelId>0</channelId>
    <bright>128</bright>
    <contrast>128</contrast>
    <saturation>128</saturation>
    <hue>128</hue>
    <sharpen>128</sharpen>
    </VideoInput>
    <InputAdvanceCfg version="1.1">
    <channelId>0</channelId>
    <digitalChannel>1</digitalChannel>
    <PowerLineFrequency>
    <mode>50hz</mode>
    <enable>0</enable>
    </PowerLineFrequency>
    <Exposure>
    <mode>auto</mode>
    <Gainctl>
    <defMin>1</defMin>
    <defMax>100</defMax>
    <curMin>1</curMin>
    <curMax>62</curMax>
    </Gainctl>
    <Shutterctl>
    <defMin>0</defMin>
    <defMax>125</defMax>
    <curMin>0</curMin>
    <curMax>125</curMax>
    </Shutterctl>
    <shutterLevel>1/30</shutterLevel>
    <gainLevel>50</gainLevel>
    </Exposure>
    <Scene>
    <mode>auto</mode>
    <modeList>auto, manual</modeList>
    <Redgain>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </Redgain>
    <Bluegain>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </Bluegain>
    </Scene>
    <DayNight>
    <mode>auto</mode>
    <IrcutMode>ir</IrcutMode>
    <Threshold>medium</Threshold>
    </DayNight>
    <BLC>
    <enable>0</enable>
    <mode>backLight</mode>
    <backlight>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </backlight>
    <dynamicrange>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </dynamicrange>
    </BLC>
    <mirror>0</mirror>
    <flip>0</flip>
    <Iris>
    <enable>0</enable>
    <state>success</state>
    <focusAutoiris>0</focusAutoiris>
    </Iris>
    <nr3d>
    <value>high</value>
    <enable>1</enable>
    </nr3d>
    </InputAdvanceCfg>
    </body>
    ```

- 31: Start Motion Alarm

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 1f | 00 00 00 00    | 05 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 1f | 00 00 00 00    | 05 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

  - **Notes:** Some cameras will not send message 33 to the client until
    after this msg has been received

- 33: `<AlarmEventList>`

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 21 | 00 00 00 f0    | 05 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <AlarmEventList version="1.1">
    <AlarmEvent version="1.1">
    <channelId>0</channelId>
    <status>MD</status>
    <recording>0</recording>
    <timeStamp>0</timeStamp>
    </AlarmEvent>
    </AlarmEventList>
    </body>
    ```

- 42: `<Email>`

  - Client

    Standard header only

  - Camera

    - XML in main payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Email version="1.1">
    <smtpServer>smtp.gmail.com</smtpServer>
    <userName></userName>
    <senderMaxLen>127</senderMaxLen>
    <password></password>
    <address1></address1>
    <address2></address2>
    <address3></address3>
    <sendNickname></sendNickname>
    <smtpPort>465</smtpPort>
    <attachment>1</attachment>
    <attachmentType>picture</attachmentType>
    <textType>withText</textType>
    <ssl>1</ssl>
    <interval>30</interval>
    </Email>
    </body>
    ```

- 43: `<Email> (write)`

  - Client
    Usual header

    - Main (gmail)

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <Email version="1.1">
      <smtpServer>smtp.gmail.com</smtpServer>
      <userName>abc@b.com</userName>
      <password>as</password>
      <address1>abc@b.com</address1>
      <address2></address2>
      <address3></address3>
      <smtpPort>465</smtpPort>
      <sendNickname>bname</sendNickname>
      <attachment>1</attachment>
      <attachmentType>picture</attachmentType>
      <textType>withText</textType>
      <ssl>1</ssl>
      <interval>30</interval>
      </Email>
      </body>
      ```

    - Main (Other)
      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <Email version="1.1">
      <smtpServer>test</smtpServer>
      <userName>abc@b.com</userName>
      <password>as</password>
      <address1>abc@b.com</address1>
      <address2></address2>
      <address3></address3>
      <smtpPort>465</smtpPort>
      <sendNickname>bname</sendNickname>
      <attachment>1</attachment>
      <attachmentType>picture</attachmentType>
      <textType>withText</textType>
      <ssl>1</ssl>
      <interval>30</interval>
      </Email>
      </body>
      ```

- 44: `<OsdChannelName>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 2c | 00 00 00 68    | 30 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 2c | 00 00 01 df    | 30 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <OsdChannelName version="1.1">
    <channelId>0</channelId>
    <name>Cammy02</name>
    <enable>1</enable>
    <topLeftX>65536</topLeftX>
    <topLeftY>65536</topLeftY>
    <enWatermark>0</enWatermark>
    <enBgcolor>0</enBgcolor>
    </OsdChannelName>
    <OsdDatetime version="1.1">
    <channelId>0</channelId>
    <enable>1</enable>
    <topLeftX>65537</topLeftX>
    <topLeftY>1</topLeftY>
    <width>0</width>
    <height>0</height>
    <language>Chinese</language>
    </OsdDatetime>
    </body>
    ```

- 45: `<OsdChannelName>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 2d | 00 00 02 23    | 32 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <OsdChannelName version="1.1">
    <channelId>0</channelId>
    <name>Cammy02</name>
    <enable>0</enable>
    <topLeftX>65536</topLeftX>
    <topLeftY>65536</topLeftY>
    <enBgcolor>0</enBgcolor>
    <enWatermark>0</enWatermark>
    </OsdChannelName>
    <OsdDatetime version="1.1">
    <channelId>0</channelId>
    <enable>1</enable>
    <topLeftX>65537</topLeftX>
    <topLeftY>1</topLeftY>
    <language>Chinese</language>
    </OsdDatetime>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 2d | 00 00 00 00    | 32 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 52: `<Shelter>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 34 | 00 00 00 68    | 36 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 34 | 00 00 00 96    | 36 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Shelter version="1.1">
    <channelId>0</channelId>
    <enable>0</enable>
    <shelterList />
    </Shelter>
    </body>
    ```

- 53: `<Shelter>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 35 | 00 00 01 d7    | 38 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Shelter version="1.1">
    <channelId>0</channelId>
    <enable>1</enable>
    <ShelterList>
    <Shelter>
    <id>0</id>
    <enable>0</enable>
    </Shelter>
    <Shelter>
    <id>1</id>
    <enable>0</enable>
    </Shelter>
    <Shelter>
    <id>2</id>
    <enable>0</enable>
    </Shelter>
    <Shelter>
    <id>3</id>
    <enable>0</enable>
    </Shelter>
    </ShelterList>
    </Shelter>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 35 | 00 00 00 00    | 38 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 54: `<RecordCfg>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 36 | 00 00 00 68    | 14 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 36 | 00 00 00 ed    | 14 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <RecordCfg version="1.1">
    <channelId>0</channelId>
    <cycle>1</cycle>
    <recordDelayTime>15</recordDelayTime>
    <preRecordTime>10</preRecordTime>
    <packageTime>5</packageTime>
    </RecordCfg>
    </body>
    ```

- 55: `<RecordCfg>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 37 | 00 00 01 3b    | 16 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <RecordCfg version="1.1">
    <cycle>0</cycle>
    <recordDelayTime>15</recordDelayTime>
    <preRecordTime>1</preRecordTime>
    <packageTime>5</packageTime>
    </RecordCfg>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 37 | 00 00 00 00    | 16 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 56: `<Compression>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 38 | 00 00 00 68    | 1d 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 38 | 00 00 03 61    | 1d 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Compression version="1.1">
    <channelId>0</channelId>
    <isNoTranslateFrame>1</isNoTranslateFrame>
    <mainStream>
    <audio>1</audio>
    <resolutionName>2304*1296</resolutionName>
    <width>2304</width>
    <height>1296</height>
    <encoderType>cbr</encoderType>
    <frame>15</frame>
    <bitRate>2560</bitRate>
    <encoderProfile>high</encoderProfile>
    </mainStream>
    <subStream>
    <audio>1</audio>
    <resolutionName>896*512</resolutionName>
    <width>896</width>
    <height>512</height>
    <encoderType>cbr</encoderType>
    <frame>15</frame>
    <bitRate>512</bitRate>
    <encoderProfile>high</encoderProfile>
    </subStream>
    <thirdStream>
    <audio>0</audio>
    <resolutionName></resolutionName>
    <width>0</width>
    <height>0</height>
    <encoderType>vbr</encoderType>
    <frame>0</frame>
    <bitRate>0</bitRate>
    <encoderProfile>default</encoderProfile>
    </thirdStream>
    </Compression>
    </body>
    ```

- 57: `<Compression>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 39 | 00 00 02 bc    | 1f 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Compression version="1.1">
    <channelId>0</channelId>
    <mainStream>
    <audio>0</audio>
    <resolutionName>2304*1296</resolutionName>
    <width>2304</width>
    <height>1296</height>
    <encoderType>vbr</encoderType>
    <frame>15</frame>
    <bitRate>2560</bitRate>
    <encoderProfile>high</encoderProfile>
    </mainStream>
    <subStream>
    <audio>0</audio>
    <resolutionName>896*512</resolutionName>
    <width>896</width>
    <height>512</height>
    <encoderType>vbr</encoderType>
    <frame>15</frame>
    <bitRate>512</bitRate>
    <encoderProfile>high</encoderProfile>
    </subStream>
    </Compression>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 39 | 00 00 00 00    | 1f 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 58: `<AbilitySupport>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 3a | 00 00 00 6b    | 03 00 00 00       | 00 00       | 64 14         | 00 00 00 6b    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <userName>PlainTextUsername</userName>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 3a | 00 00 03 a4    | 03 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <AbilitySuppport version="1.1">
    <userName></userName>
    <system>1</system>
    <streaming>1</streaming>
    <record>1</record>
    <network>1</network>
    <PTZ>1</PTZ>
    <IO>0</IO>
    <alarm>1</alarm>
    <image>1</image>
    <video>1</video>
    <audio>1</audio>
    <security>1</security>
    <replay>1</replay>
    <disk>1</disk>
    </AbilitySuppport>
    <UserList version="1.1">
    <User>
    <userId>0</userId>
    <userName>PlainTextUsername</userName>
    <password>PlainTextPASSWORD</password>
    <userLevel>1</userLevel>
    <loginState>0</loginState>
    <userSetState>none</userSetState>
    </User>
    <User>
    <userId>0</userId>
    <userName>PlainTextUsername</userName>
    <password>PlainTextPASSWORD]VX</password>
    <userLevel>0</userLevel>
    <loginState>0</loginState>
    <userSetState>none</userSetState>
    </User>
    <User>
    <userId>0</userId>
    <userName>PlainTextUsername</userName>
    <password>PlainTextPASSWORD</password>
    <userLevel>1</userLevel>
    <loginState>1</loginState>
    <userSetState>none</userSetState>
    </User>
    </UserList>
    </body>
    ```

  - **Notes:** The passwords are not sent in some models of cameras namely
    RLC-410 4mp, RLC-410 5mp, RLC-520 (fw 200710), RLC-811A in these cases the
    passwords are blank. In some older cameras that do not use encryption at
    all these passwords are completely visible to any network sniffers. Even
    the "encrypted" cameras only have weak encryption that is easily broken
    since the decryption key is fixed and well-known.

- 59: `<UserList>`

  - Client

    - Header: Standard header

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <userName>admin</userName>
    </Extension>
    ```

  - Camera

    - Header: Standard header

    - Payload — Changing the password of testUser

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <UserList version="1.1">
    <User>
    <userName>admin</userName>
    <password>password12</password>
    <userLevel>1</userLevel>
    <userSetState>none</userSetState>
    </User>
    <User>
    <userName>testUser</userName>
    <password>newPass</password>
    <userLevel>0</userLevel>
    <userSetState>modify</userSetState>
    </User>
    </UserList>
    </body>
    ```

    - Payload — creating a user

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <UserList version="1.1">
    <User>
    <userName>admin</userName>
    <password>password12</password>
    <userLevel>1</userLevel>
    <userSetState>none</userSetState>
    </User>
    <User>
    <userName>testUser</userName>
    <password>testPass</password>
    <userLevel>0</userLevel>
    <userSetState>add</userSetState>
    </User>
    </UserList>
    </body>
    ```

    - Payload — deleting a user

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <UserList version="1.1">
    <User>
    <userName>admin</userName>
    <password>password12</password>
    <userLevel>1</userLevel>
    <userSetState>none</userSetState>
    </User>
    <User>
    <userName>testUser</userName>
    <password>newPass</password>
    <userLevel>0</userLevel>
    <userSetState>delete</userSetState>
    </User>
    </UserList>
    </body>
    ```

  - **Notes:** The passwords are not sent in some models of cameras namely
    RLC-410 4mp, RLC-410 5mp, RLC-520 (fw 200710), RLC-811A in these cases the
    passwords are blank. In some older cameras that do not use encryption at
    all these passwords are completely visible to any network sniffers. Even
    the "encrypted" cameras only have weak encryption that is easily broken
    since the decryption key is fixed and well-known.
  - **Notes:** The password field seems to only be needed when creating a user
    or changing a users password.
  - **Notes:** It does not appear like userLevel is modifiable (at least on
    RLC-811A)

- 67: `<ConfigFileInfo> (FW Upgrade)`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 43 | 00 00 01 00    | 00 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <ConfigFileInfo version="1.1">
    <fileName>FIRMWAREFILE.pak</fileName>
    <fileSize>SIZE_IN_BYTES</fileSize>
    <updateParameter>0</updateParameter>
    </ConfigFileInfo>
    </body>
    ```

  - **Notes:** updateParameter refers to updating the settings. If 1 it will restore factory settings. If 0 it will keep current.

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 43 | 00 00 00 00    | 00 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 43 | 00 00 94 58    | 00 00 00 00       | 00 00       | 64 14         | 00 00 00 6a    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <binaryData>1</binaryData>
    </Extension>
    ```

    - Payload

      This contains binary data of the file but stops once the message size reaches
      38000 bytes and continues in another packet. There does not appear to
      be a checksum or hash and this part contains only the raw bytes of the file.

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 43 | 00 00 00 00    | 00 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

  - **Notes:** Last two messages repeat until all data is sent

- 76: `<Ip>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 4c | 00 00 00 00    | 22 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 4c | 00 00 01 69    | 22 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Dhcp version="1.1">
    <enable>1</enable>
    </Dhcp>
    <AutoDns version="1.1">
    <enable>1</enable>
    </AutoDns>
    <Ip version="1.1">
    <ip>192.168.1.101</ip>
    <mask>255.255.255.0</mask>
    <mac>94:E0:D6:E9:89:86</mac>
    <gateway>192.168.1.1</gateway>
    </Ip>
    <Dns version="1.1">
    <dns1>1.1.1.1</dns1>
    <dns2>8.8.8.8</dns2>
    </Dns>
    </body>
    ```

- 77: `<Ip>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 4d | 00 00 01 5b    | 25 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Dhcp version="1.1">
    <enable>1</enable>
    </Dhcp>
    <AutoDns>
    <enable>0</enable>
    </AutoDns>
    <Ip version="1.1">
    <ip>192.168.1.101</ip>
    <mask>255.255.255.0</mask>
    <mac>94:E0:D6:E9:89:86</mac>
    <gateway>192.168.1.1</gateway>
    </Ip>
    <Dns version="1.1">
    <dns1>1.1.1.1</dns1>
    <dns2>8.8.8.8</dns2>
    </Dns>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 4d | 00 00 00 00    | 14 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 78: `<VideoInput>`

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 4e | 00 00 00 d3    | 1b 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <VideoInput version="1.1">
    <channelId>0</channelId>
    <bright>128</bright>
    <contrast>128</contrast>
    <saturation>128</saturation>
    <hue>128</hue>
    </VideoInput>
    </body>
    ```

- 79: `<Serial>` (ptz)

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 4f | 00 00 01 3b    | 1b 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Serial version="1.1">
    <channelId>0</channelId>
    <baudRate>9600</baudRate>
    <dataBit>CS8</dataBit>
    <stopBit>1</stopBit>
    <parity>none</parity>
    <flowControl>none</flowControl>
    <controlProtocol>PELCO_D</controlProtocol>
    <controlAddress>1</controlAddress>
    </Serial>
    </body>
    ```

- 80: `<VersionInfo>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 50 | 00 00 00 00    | 08 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 50 | 00 00 01 f0    | 08 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <VersionInfo version="1.1">
    <name>Cammy02</name>
    <type>E1</type>
    <serialNumber>00000000000000</serialNumber>
    <buildDay>build 19110800</buildDay>
    <hardwareVersion>IPC_517SD5</hardwareVersion>
    <cfgVersion>v2.0.0.0</cfgVersion>
    <firmwareVersion>v2.0.0.587_19110800</firmwareVersion>
    <detail>IPC_51716M110000000100000</detail>
    <IEClient>IEClient</IEClient>
    <pakSuffix>pak</pakSuffix>
    <helpVersion>blackPointsLevel=0</helpVersion>
    </VersionInfo>
    </body>
    ```

- 81: `<Record>` (schedule)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 51 | 00 00 00 68    | 19 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 51 | 00 00 04 30    | 19 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Record version="1.1">
    <channelId>0</channelId>
    <enable>1</enable>
    <ScheduleList>
    <Schedule>
    <alarmType>MD</alarmType>
    <timeBlockList>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Sunday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Monday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Wednesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Thursday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Friday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Saturday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    </timeBlockList>
    </Schedule>
    </ScheduleList>
    </Record>
    </body>
    ```

- 82: `<Record>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 52 | 00 00 05 da    | 1a 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Record version="1.1">
    <channelId>0</channelId>
    <enable>1</enable>
    <ScheduleList>
    <Schedule>
    <alarmType>MD</alarmType>
    <timeBlockList>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Sunday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Monday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>12</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>14</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Wednesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Thursday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Friday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Saturday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    </timeBlockList>
    </Schedule>
    <Schedule>
    <alarmType>none</alarmType>
    <timeBlockList>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>13</beginHour>
    <endHour>13</endHour>
    </timeBlock>
    </timeBlockList>
    </Schedule>
    </ScheduleList>
    </Record>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 52 | 00 00 00 00    | 1a 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 93: `<LinkType>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 5d | 00 00 00 00    | 17 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

- 102: `<HDDInfoList>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 66 | 00 00 00 00    | 07 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 66 | 00 00 00 55    | 07 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <HddInfoList version="1.1" />
    </body>
    ```

- 104: `<SystemGeneral>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 68 | 00 00 00 00    | 0a 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 68 | 00 00 01 a5    | 0a 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <SystemGeneral version="1.1">
    <timeZone>-25200</timeZone>
    <osdFormat>DMY</osdFormat>
    <year>2020</year>
    <month>10</month>
    <day>6</day>
    <hour>18</hour>
    <minute>36</minute>
    <second>34</second>
    <deviceId>0</deviceId>
    <timeFormat>0</timeFormat>
    <language>English</language>
    <deviceName>Cammy02</deviceName>
    </SystemGeneral>
    <Norm version="1.1">
    <norm>NTSC</norm>
    </Norm>
    </body>
    ```

- 109: `<Snap>`

  - Client:
    Usual header

    - Meta:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Main:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Snap version="1.1">
    <channelId>0</channelId>
    <logicChannel>0</logicChannel>
    <time>0</time>
    <fullFrame>0</fullFrame>
    <streamType>main</streamType>
    </Snap>
    </body>
    ```

    - Camera:

    **Notes:** XML & Binary reply over mutliple packets:

    - Reply 1:

      - Main:

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <Snap version="1.1">
      <channelId>0</channelId>
      <fileName>01_20230518140240.jpg</fileName>
      <time>0</time>
      <pictureSize>23644</pictureSize>
      </Snap>
      </body>
      ```

    - Reply 2:
      - Meta:
      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <Extension version="1.1">
      <binaryData>1</binaryData>
      </Extension>
      ```
      - Main:
        **Binary data containing the file may be broken over multiple packets**

- 115: `<WifiSignal>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 73 | 00 00 00 00    | 0c 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 73 | 00 00 00 75    | 0c 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <WifiSignal version="1.1">
    <signal>-40</signal>
    </WifiSignal>
    </body>
    ```

- 116: `<Wifi>`

  - Client:
    Usual header

  - Camera:
    Payload:
    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Wifi version="1.1">
    <mode>station</mode>
    <authMode>wpa2psk</authMode>
    <encryptType>aes</encryptType>
    <udidList>
        <udid>
        <name>WIFINAME</name>
        <signal>NUMBER</signal>
        <encrypt>ONE_OR_ZERO</encrypt>
        </udid>
        # Repeats for ALL wifi in range
    </udidList>
    <ssid>YOUR_CURRENT_WIFI</ssid>
    <key>YOUR_CURRENT_WIFI_PASSWORD_UNENCRYPTED</key>
    <channel>YOUR_CURRENT_WIFI_CHANNEL</channel>
    </Wifi>
    </body>
    ```

- 132: `<VideoInput>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 84 | 00 00 00 68    | 65 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 84 | 00 00 05 7c    | 65 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <VideoInput version="1.1">
    <channelId>0</channelId>
    <bright>128</bright>
    <contrast>128</contrast>
    <saturation>128</saturation>
    <hue>128</hue>
    <sharpen>128</sharpen>
    </VideoInput>
    <InputAdvanceCfg version="1.1">
    <channelId>0</channelId>
    <digitalChannel>1</digitalChannel>
    <PowerLineFrequency>
    <mode>50hz</mode>
    <enable>0</enable>
    </PowerLineFrequency>
    <Exposure>
    <mode>auto</mode>
    <Gainctl>
    <defMin>1</defMin>
    <defMax>100</defMax>
    <curMin>1</curMin>
    <curMax>62</curMax>
    </Gainctl>
    <Shutterctl>
    <defMin>0</defMin>
    <defMax>125</defMax>
    <curMin>0</curMin>
    <curMax>125</curMax>
    </Shutterctl>
    <shutterLevel>1/30</shutterLevel>
    <gainLevel>50</gainLevel>
    </Exposure>
    <Scene>
    <mode>auto</mode>
    <modeList>auto, manual</modeList>
    <Redgain>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </Redgain>
    <Bluegain>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </Bluegain>
    </Scene>
    <DayNight>
    <mode>auto</mode>
    <IrcutMode>ir</IrcutMode>
    <Threshold>medium</Threshold>
    </DayNight>
    <BLC>
    <enable>0</enable>
    <mode>backLight</mode>
    <backlight>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </backlight>
    <dynamicrange>
    <min>0</min>
    <max>255</max>
    <cur>128</cur>
    </dynamicrange>
    </BLC>
    <mirror>0</mirror>
    <flip>0</flip>
    <Iris>
    <enable>0</enable>
    <state>success</state>
    <focusAutoiris>0</focusAutoiris>
    </Iris>
    <nr3d>
    <value>high</value>
    <enable>1</enable>
    </nr3d>
    </InputAdvanceCfg>
    </body>
    ```

- 124: `<PushInfo>`

  - Client

    - Payload:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <PushInfo version="1.1">
    <token>A_PUSH_NOTIFICATION_TOKEN</token>
    <phoneType>reo_iphone</phoneType>
    <clientID>A_PUSH_NOTIFICATION_CLIENTID</clientID>
    </PushInfo>
    </body>
    ```

  - Camera

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <PushRspInfo version="1.1">
    <registerHandle>-1</registerHandle>
    <uid>CAMERA_UID</uid>
    <uidKey>A_4_CHAR_STRING</uidKey>
    </PushRspInfo>
    </body>
    ```

- 133: `<RfAlarm>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 85 | 00 00 00 00    | 06 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 85 | 00 00 00 7f    | 06 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <RfAlarm version="1.1">
    <enable>1</enable>
    <type>none</type>
    </RfAlarm>
    </body>
    ```

- 141: `<Email> (test)`

  - Client

    Usual Header

    - Main

      Sends the same XML as that on `<Email> (write)` (MsgId 43)

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <Email version="1.1">
      <smtpServer>192.168.1.15</smtpServer>
      <userName>Test@Cammy01.neolink</userName>
      <password>TestPass</password>
      <address1>postman@nowhere</address1>
      <address2></address2>
      <address3></address3>
      <smtpPort>22022</smtpPort>
      <sendNickname>TestUser</sendNickname>
      <attachment>1</attachment>
      <attachmentType>picture</attachmentType>
      <textType>withText</textType>
      <ssl>0</ssl>
      <interval>30</interval>
      </Email>
      </body>
      <?xml version="1.0" encoding="UTF-8" ?>
      ```
  - Camera

    - Head Only Reply

    Seems to return 400 if send failed

    Channel_id seems to be set to -106 on failure maybe an internal error code

- 146: `<StreamInfoList>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 92 | 00 00 00 00    | 04 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 92 | 00 00 02 fc    | 04 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <StreamInfoList version="1.1">
    <StreamInfo>
    <channelBits>1</channelBits>
    <encodeTable>
    <type>mainStream</type>
    <resolution>
    <width>2304</width>
    <height>1296</height>
    </resolution>
    <defaultFramerate>15</defaultFramerate>
    <defaultBitrate>2560</defaultBitrate>
    <framerateTable>15,12,10,8,6,4,2</framerateTable>
    <bitrateTable>1024,1536,2048,2560,3072</bitrateTable>
    </encodeTable>
    <encodeTable>
    <type>subStream</type>
    <resolution>
    <width>896</width>
    <height>512</height>
    </resolution>
    <defaultFramerate>15</defaultFramerate>
    <defaultBitrate>512</defaultBitrate>
    <framerateTable>15,12,10,8,6,4,2</framerateTable>
    <bitrateTable>128,256,384,512,768,1024</bitrateTable>
    </encodeTable>
    </StreamInfo>
    </StreamInfoList>
    </body>
    ```

- 151: `<AbilityInfo>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 97 | 00 00 00 a7    | 02 00 00 00       | 00 00       | 64 14         | 00 00 00 a7    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <userName>PlainTextUsername</userName>
    <token>system, network, alarm, record, video, image</token>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 97 | 00 00 03 ac    | 02 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <AbilityInfo version="1.1">
    <userName>PlainTextUsername</userName>
    <system>
    <subModule>
    <abilityValue>general_rw, norm_rw, version_ro, uid_ro, autoReboot_rw, restore_rw, reboot_rw, shutdown_rw, dst_rw, log_ro, performance_ro, upgrade_rw, export_rw, import_rw, bootPwd_rw</abilityValue>
    </subModule>
    </system>
    <network>
    <subModule>
    <abilityValue>port_rw, dns_rw, email_rw, ipFilter_rw, localLink_rw, pppoe_rw, upnp_rw, wifi_rw, ntp_rw, netStatus_rw, ptop_rw, autontp_rw</abilityValue>
    </subModule>
    </network>
    <alarm>
    <subModule>
    <channelId>0</channelId>
    <abilityValue>motion_rw</abilityValue>
    </subModule>
    </alarm>
    <image>
    <subModule>
    <channelId>0</channelId>
    <abilityValue>ispBasic_rw, ispAdvance_rw, ledState_rw</abilityValue>
    </subModule>
    </image>
    <video>
    <subModule>
    <channelId>0</channelId>
    <abilityValue>osdName_rw, osdTime_rw, shelter_rw</abilityValue>
    </subModule>
    </video>
    </AbilityInfo>
    </body>
    ```

- 190: PTZ Preset

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 be | 00 00 00 68    | 0d 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 be | 00 00 00 86    | 0d 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <PtzPreset version="1.1">
    <channelId>0</channelId>
    <presetList>
    <preset>
    <id>0</id>
    <name>Test</name>
    </preset>
    </presetList>
    </PtzPreset>
    </body>
    ```

- 192:

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 c0 | 00 00 00 00    | 05 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 c0 | 00 00 00 00    | 05 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 199: `<Support>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 c7 | 00 00 00 00    | 02 00 00 00       | 00 00       | 64 14         | 00 00 00 00    |

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 c7 | 00 00 05 f6    | 02 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <Support version="1.1">
    <IOInputPortNum>0</IOInputPortNum>
    <IOOutputPortNum>0</IOOutputPortNum>
    <diskNum>0</diskNum>
    <channelNum>1</channelNum>
    <audioNum>1</audioNum>
    <ptzMode>pt</ptzMode>
    <ptzCfg>0</ptzCfg>
    <B485>0</B485>
    <autoUpdate>0</autoUpdate>
    <pushAlarm>1</pushAlarm>
    <ftp>0</ftp>
    <ftpTest>1</ftpTest>
    <email>1</email>
    <wifi>5</wifi>
    <record>0</record>
    <wifiTest>1</wifiTest>
    <rtsp>0</rtsp>
    <onvif>0</onvif>
    <audioTalk>1</audioTalk>
    <rfVersion>0</rfVersion>
    <rtmp>0</rtmp>
    <noExternStream>1</noExternStream>
    <timeFormat>1</timeFormat>
    <ddnsVersion>1</ddnsVersion>
    <emailVersion>3</emailVersion>
    <pushVersion>1</pushVersion>
    <pushType>1</pushType>
    <audioAlarm>1</audioAlarm>
    <apMode>0</apMode>
    <cloudVersion>30</cloudVersion>
    <replayVersion>1</replayVersion>
    <mobComVersion>0</mobComVersion>
    <syncTime>1</syncTime>
    <netPort>1</netPort>
    <videoStandard>0</videoStandard>
    <smartHome>
    <version>1</version>
    <item>
    <name>googleHome</name>
    <ver>1</ver>
    </item>
    <item>
    <name>amazonAlexa</name>
    <ver>1</ver>
    </item>
    </smartHome>
    <item>
    <chnID>0</chnID>
    <ptzType>3</ptzType>
    <ptzPreset>0</ptzPreset>
    <ptzPatrol>0</ptzPatrol>
    <ptzTattern>0</ptzTattern>
    <ptzControl>0</ptzControl>
    <rfCfg>0</rfCfg>
    <noAudio>0</noAudio>
    <autoFocus>0</autoFocus>
    <videoClip>0</videoClip>
    <battery>0</battery>
    <ispCfg>0</ispCfg>
    <osdCfg>1</osdCfg>
    <batAnalysis>0</batAnalysis>
    <dynamicReso>0</dynamicReso>
    <audioVersion>15</audioVersion>
    <ledCtrl>1</ledCtrl>
    <motion>1</motion>
    </item>
    </Support>
    </body>
    ```

    This example payload is from a duel camera with zoom on the secondary camera

    ```xml
    <Support version=\"1.1\">
    <IOInputPortNum>0</IOInputPortNum>
    <IOOutputPortNum>0</IOOutputPortNum>
    <diskNum>0</diskNum>
    <channelNum>2</channelNum>
    <audioNum>1</audioNum>
    <ptzMode>pt</ptzMode>
    <ptzCfg>0</ptzCfg>
    <B485>0</B485>
    <autoUpdate>1</autoUpdate>
    <pushAlarm>3</pushAlarm>
    <ftp>0</ftp>
    <ftpTest>1</ftpTest>
    <email>1</email>
    <wifi>0</wifi>
    <record>0</record>
    <wifiTest>1</wifiTest>
    <rtsp>0</rtsp>
    <onvif>0</onvif>
    <audioTalk>1</audioTalk>
    <rfVersion>4</rfVersion>
    <rtmp>0</rtmp>
    <noExternStream>1</noExternStream>
    <timeFormat>1</timeFormat>
    <ddnsVersion>1</ddnsVersion>
    <emailVersion>3</emailVersion>
    <pushVersion>1</pushVersion>
    <pushType>1</pushType>
    <audioAlarm>1</audioAlarm>
    <apMode>0</apMode>
    <cloudVersion>127</cloudVersion>
    <replayVersion>20</replayVersion>
    <mobComVersion>3</mobComVersion>
    <ExportImport>3</ExportImport>
    <languageVer>0</languageVer>
    <videoStandard>0</videoStandard>
    <syncTime>0</syncTime>
    <netPort>1</netPort>
    <nasVersion>7</nasVersion>
    <needReboot>0</needReboot>
    <reboot>1</reboot>
    <audioCfg>1</audioCfg>
    <networkDiagnosis>0</networkDiagnosis>
    <heightDiffAdjust>2</heightDiffAdjust>
    <upgrade>1</upgrade>
    <gps>0</gps>
    <powerSavingCfg>0</powerSavingCfg>
    <loginLocked>0</loginLocked>
    <viewPlan>0</viewPlan>
    <previewReplayLimit>0</previewReplayLimit>
    <IOTLink>0</IOTLink>
    <IOTLinkActionMax>48</IOTLinkActionMax>
    <recordCfg>53</recordCfg>
    <largeBattery>0</largeBattery>
    <smartHome>
    	<version>1</version>
    	<item>
    		<name>googleHome</name>
    		<ver>1</ver>
    	</item>
    	<item>
    		<name>amazonAlexa</name>
    		<ver>1</ver>
    	</item>
    </smartHome>
    <item>
    	<chnID>0</chnID>
    	<ptzType>3</ptzType>
    	<rfCfg>0</rfCfg>
    	<noAudio>0</noAudio>
    	<autoFocus>0</autoFocus>
    	<videoClip>0</videoClip>
    	<battery>2</battery>
    	<ispCfg>195</ispCfg>
    	<osdCfg>1</osdCfg>
    	<batAnalysis>1</batAnalysis>
    	<dynamicReso>1</dynamicReso>
    	<audioVersion>63</audioVersion>
    	<ledCtrl>10</ledCtrl>
    	<ptzControl>207</ptzControl>
    	<newIspCfg>22467</newIspCfg>
    	<ptzPreset>1</ptzPreset>
    	<ptzPatrol>0</ptzPatrol>
    	<ptzTattern>0</ptzTattern>
    	<autoPt>0</autoPt>
    	<h264Profile>7</h264Profile>
    	<motion>0</motion>
    	<aitype>32503</aitype>
    	<aiAnimalType>1</aiAnimalType>
    	<timelapse>3</timelapse>
    	<snap>20</snap>
    	<encCtrl>0</encCtrl>
    	<zfBacklash>0</zfBacklash>
    	<IOTLinkAbility>747</IOTLinkAbility>
    	<ipcAudioTalk>1</ipcAudioTalk>
    	<binoCfg>0</binoCfg>
    	<thumbnail>2</thumbnail>
    </item>
    <item>
    	<chnID>1</chnID>
    	<ptzType>3</ptzType>
    	<rfCfg>0</rfCfg>
    	<noAudio>0</noAudio>
    	<autoFocus>0</autoFocus>
    	<videoClip>0</videoClip>
    	<battery>2</battery>
    	<ispCfg>195</ispCfg>
    	<osdCfg>1</osdCfg>
    	<batAnalysis>1</batAnalysis>
    	<dynamicReso>0</dynamicReso>
    	<audioVersion>63</audioVersion>
    	<ledCtrl>10</ledCtrl>
    	<ptzControl>223</ptzControl>
    	<newIspCfg>22467</newIspCfg>
    	<ptzPreset>1</ptzPreset>
    	<ptzPatrol>0</ptzPatrol>
    	<ptzTattern>0</ptzTattern>
    	<autoPt>0</autoPt>
    	<h264Profile>7</h264Profile>
    	<motion>0</motion>
    	<aitype>32503</aitype>
    	<aiAnimalType>1</aiAnimalType>
    	<timelapse>3</timelapse>
    	<snap>28</snap>
    	<encCtrl>0</encCtrl>
    	<zfBacklash>0</zfBacklash>
    	<IOTLinkAbility>747</IOTLinkAbility>
    	<ipcAudioTalk>1</ipcAudioTalk>
    	<binoCfg>0</binoCfg>
    	<thumbnail>2</thumbnail>
    </item>
    </Support>
    ```

- 201: `<TalkConfig>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Binary Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | ------------- |
      | 0a bc de f0 | 00 00 00 c9 | 00 00 01 f2    | 12 00 00 00       | 00 00       | 64 14         | 00 00 00 68   |

    - Body

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Binary

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <TalkConfig version="1.1">
    <channelId>0</channelId>
    <duplex>FDX</duplex>
    <audioStreamMode>followVideoStream</audioStreamMode>
    <audioConfig>
    <audioType>adpcm</audioType>
    <sampleRate>16000</sampleRate>
    <samplePrecision>16</samplePrecision>
    <lengthPerEncoder>1024</lengthPerEncoder>
    <soundTrack>mono</soundTrack>
    </audioConfig>
    </TalkConfig>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Binary Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | ------------- |
      | 0a bc de f0 | 00 00 00 c9 | 00 00 00 00    | 12 00 00 00       | c8 00       | 00 00         | 00 00 00 00   |

- 202: `Talk`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Binary Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | ------------- |
      | 0a bc de f0 | 00 00 00 ca | 00 00 08 c3    | 00 00 00 00       | 00 00       | 64 14         | 00 00 00 83   |

    - Body

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <binaryData>1</binaryData>
    <channelId>0</channelId>
    </Extension>
    ```

    - Binary

      Binary data contains media-packets of adpcm data

  **Notes**: No reply from camera. After this the client keeps sending this packet with binary in the BcMedia encoded packets of adpcm data

- 204: `<rfAlarmCfg>` (write)

  - Client

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <rfId>0</rfId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <rfAlarmCfg version="1.1">
    <rfID>0</rfID>
    <enable>1</enable>
    <sensitivity>1</sensitivity>
    <sensiValue>16</sensiValue> <!-- 11 is 90? this doesn't seem like a straight-up value to set... -->
    <reduceFalseAlarm>0</reduceFalseAlarm>
    <timeBlockList>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Sunday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Monday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Wednesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Thursday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Friday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>0</enable>
    <weekDay>Saturday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    </timeBlockList>
    <alarmHandle>
    <item>
    <channel>0</channel>
    <handleType>none</handleType>
    </item>
    </alarmHandle>
    </rfAlarmCfg>
    </body>
    ```

  **Notes**: Used for motion config on at least Reolink Floodlight.

- 208: `<LedState>`

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 d0 | 00 00 00 68    | 2e 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 d0 | 00 00 00 c2    | 2e 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <LedState version="1.1">
    <channelId>0</channelId>
    <ledVersion>2</ledVersion>
    <state>auto</state>
    <lightState>open</lightState>
    </LedState>
    </body>
    ```

- 209: `<LedState>` (write)

  - Client

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 d1 | 00 00 01 10    | 85 00 00 00       | 00 00       | 64 14         | 00 00 00 68    |

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <LedState version="1.1">
    <channelId>0</channelId>
    <state>close</state>
    <lightState>open</lightState>
    </LedState>
    </body>
    ```

  - Camera

    - Header

      | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
      | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
      | 0a bc de f0 | 00 00 00 d1 | 00 00 00 00    | 85 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

- 216: `<EmailTask> (write)`

  - Client
    Usual header

    - Meta:

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <Extension version="1.1">
      <channelId>0</channelId>
      </Extension>
      ```

    - Main (Turn ON)

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <EmailTask version="1.1">
      <channelId>0</channelId>
      <enable>1</enable>
      </EmailTask>
      </body>
      ```

    - Main (Turn OFF)

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <EmailTask version="1.1">
      <channelId>0</channelId>
      <enable>0</enable>
      </EmailTask>
      </body>
      ```

    - Main (Change)
      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <EmailTask version="1.1">
      <channelId>0</channelId>
      <enable>1</enable>
      <ScheduleList>
      <Schedule>
      <alarmType>MD</alarmType>
      <timeBlockList>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Sunday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Monday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Tuesday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Wednesday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Thursday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Friday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Saturday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      </timeBlockList>
      </Schedule>
      </ScheduleList>
      </EmailTask>
      </body>
      ```

  - Camera
    Usual header

- 217: `<EmailTask>`

  - Client
    Usual header

    - Meta:
      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <Extension version="1.1">
      <channelId>0</channelId>
      </Extension>
      ```

  - Camera
    Usual header
    - Main:
      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <EmailTask version="1.1">
      <channelId>0</channelId>
      <enable>0</enable>
      <ScheduleList>
      <Schedule>
      <alarmType>MD</alarmType>
      <timeBlockList>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Sunday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Monday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Tuesday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Wednesday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Thursday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Friday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      <timeBlock>
      <enable>1</enable>
      <weekDay>Saturday</weekDay>
      <beginHour>0</beginHour>
      <endHour>23</endHour>
      </timeBlock>
      </timeBlockList>
      </Schedule>
      </ScheduleList>
      </EmailTask>
      </body>
      ```

- 219: `<PushTask>`

  - Client
    Usual header

    - Meta:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera
    Usual header

    - Payload:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <PushTask version="1.1">
    <channelId>0</channelId>
    <enable>1</enable>
    <ScheduleList>
    <Schedule>
    <alarmType>MD</alarmType>
    <timeBlockList>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Sunday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Monday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Wednesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Thursday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Friday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Saturday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    </timeBlockList>
    </Schedule>
    </ScheduleList>
    </PushTask>
    </body>
    ```

- 232: `<AudioTask>`

  - Client
    Usual header

    - Meta:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

  - Camera
    Usual header

    - Payload:

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <AudioTask version="1.1">
    <channelId>0</channelId>
    <enable>0</enable>
    <ScheduleList>
    <Schedule>
    <alarmType>MD</alarmType>
    <timeBlockList>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Sunday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Monday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Tuesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Wednesday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Thursday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Friday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    <timeBlock>
    <enable>1</enable>
    <weekDay>Saturday</weekDay>
    <beginHour>0</beginHour>
    <endHour>23</endHour>
    </timeBlock>
    </timeBlockList>
    </Schedule>
    </ScheduleList>
    </AudioTask>
    </body>
    ```

- 264: `<audioPlayInfo>`

  - Client: Usual header and extension xml with channel

    - Payload

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <audioPlayInfo version="1.1">
      <channelId>0</channelId>
      <playMode>0</playMode>
      <playDuration>0</playDuration>
      <playTimes>1</playTimes>
      <onOff>0</onOff>
      </audioPlayInfo>
      </body>
      ```

    **Notes:** This is used to play the siren

  - Camera:

        Replies with standard OK message

- 264: `<audioCfg>` (write)

  - Client

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <audioCfg version="1.1">
    <channelId>0</channelId>
    <timeout>0</timeout>
    <audioSelect>0</audioSelect>
    <volume>85</volume>
    <preAlarm>0</preAlarm>
    <pauseAlarm>0</pauseAlarm>
    <pauseType>0</pauseType>
    <pauseStartTime>0</pauseStartTime>
    <pauseTime>0</pauseTime>
    <audioListId>0</audioListId>
    <linkageCtrlEnable>1</linkageCtrlEnable>
    </audioCfg>
    </body>
    ```

- 268: `CloudBindInfo`

  - Client:
    Usual header only packet

  - Camera

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <CloudBindInfo version="1.1">
    <binded>1</binded>
    </CloudBindInfo>
    </body>
    ```

- 282: `CloudLoginKey`

  - Client:
    Usual header only packet

  - Camera

    - Payload

    ```xml
    <body>
    <CloudLoginKey version="1.1">
    <enable>0</enable>
    </CloudLoginKey>
    </body>
    ```

- 287: `<TimeCfg>`

  - Client:

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <TimeCfg version="1.1">
    <realTime>1684393362</realTime>
    </TimeCfg>
    </body>
    ```

  - Camera
    Header only 200 Ok reply

- 288: `<FloodlightManual>` (write)

  - Client

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <FloodlightManual version="1.1">
    <channelId>0</channelId>
    <status>1</status>
    <duration>180</duration> <!-- in seconds -->
    </FloodlightManual>
    </body>
    ```

- 290: `<FloodlightTask>` (write)

  - Client

    - Extension

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <Extension version="1.1">
    <channelId>0</channelId>
    </Extension>
    ```

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <FloodlightTask version="1.1">
    <channelId>0</channelId>
    <enable>1</enable> <!-- if any of the motion/night/schedule settings are enabled -->
    <alarmMode>1</alarmMode>
    <preview_auto>0</preview_auto>
    <brightness_cur>92</brightness_cur> <!-- default brightness, or only brightness if just motion mode -->
    <duration>300</duration> <!-- seconds light stays on, at least with motion -->
    <detectType>none</detectType>
    <lastAlarmMode>2</lastAlarmMode>
    <Schedule>
    <startHour>18</startHour>
    <startMin>0</startMin>
    <endHour>6</endHour>
    <endMin>0</endMin>
    </Schedule>
    <lightSensThreshold>
    <lightCur>1000</lightCur>
    <darkCur>1900</darkCur>
    </lightSensThreshold>
    <FloodlightScheduleList />
    <nightLongViewMultiBrightness>  <!-- night mode with usual/alarm brightness -->
    <enable>0</enable>
    <alarmBrightness>
    <cur>100</cur>
    </alarmBrightness>
    <alarmDelay>
    <cur>10</cur> <!-- seconds for alarm brightness -->
    </alarmDelay>
    </nightLongViewMultiBrightness>
    </FloodlightTask>
    </body>
    ```

- 291: `<FloodlightStatusList>` (read)

  - Camera

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <FloodlightStatusList version="1.1">
    <FloodlightStatus>
    <channel>0</channel>
    <status>1</status>
    </FloodlightStatus>
    </FloodlightStatusList>
    </body>
    ```

- 294: `<PtzZoomFocus>` (Read)

  - Client: Standard header and channel extension

  - Camera:

    Usual header and Extension XML with channel

    - Payload:

      ```xml
      <?xml version="1.0" encoding="UTF-8" ?>
      <body>
      <PtzZoomFocus version="1.1">
      <channelId>1</channelId>
      <zoom>
      <maxPos>6000</maxPos>
      <minPos>1000</minPos>
      <curPos>2501</curPos>
      </zoom>
      <focus>
      <maxPos>100</maxPos>
      <minPos>1</minPos>
      <curPos>51</curPos>
      </focus>
      </PtzZoomFocus>
      </body>
      ```

- 295: `<StartZoomFocus>` (Write)

  - Client

    - Extension: Usual extension xml with channel

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <StartZoomFocus version="1.1">
    <channelId>1</channelId>
    <command>zoomPos</command>
    <movePos>2994</movePos>
    </StartZoomFocus>
    </body>
    ```

- 299: `<AiCfg>` (read)

  - Client: Standard payload with extension xml of the channel

  - Camera

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <AiCfg version="1.1">
    <channelId>0</channelId>
    <smartTrack>0</smartTrack>
    <smartTrackMode>2</smartTrackMode>
    <smartTrackModeAbility>14</smartTrackModeAbility>
    <detectType>people,vehicle,dog_cat</detectType>
    <smartTrackType>people</smartTrackType>
    <smartTrackPt>1</smartTrackPt>
    <smartTrackObjectStopDelay>20</smartTrackObjectStopDelay>
    <smartTrackObjectDisappearDelay>10</smartTrackObjectDisappearDelay>
    </AiCfg>
    </body>
    ```

- 438: `<FloodlightTask>` (read)

  - Camera

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <FloodlightTask version="1.1">
    <channel>0</channel>
    <alarmMode>1</alarmMode>
    <enable>1</enable>
    <lastAlarmMode>2</lastAlarmMode>
    <preview_auto>0</preview_auto>
    <duration>300</duration>
    <brightness_cur>92</brightness_cur>
    <brightness_max>100</brightness_max>
    <brightness_min>1</brightness_min>
    <schedule>
    <startHour>18</startHour>
    <startMin>0</startMin>
    <endHour>6</endHour>
    <endMin>0</endMin>
    </schedule>
    <lightSensThreshold>
    <min>1000</min>
    <max>2300</max>
    <lightCur>1000</lightCur>
    <darkCur>1900</darkCur>
    <lightDef>1000</lightDef>
    <darkDef>1900</darkDef>
    </lightSensThreshold>
    <FloodlightScheduleList>
    <maxNum>32</maxNum>
    </FloodlightScheduleList>
    <nightLongViewMultiBrightness>
    <enable>0</enable>
    <alarmBrightness>
    <min>1</min>
    <max>100</max>
    <cur>100</cur>
    <def>100</def>
    </alarmBrightness>
    <alarmDelay>
    <min>5</min>
    <max>600</max>
    <cur>10</cur>
    <def>10</def>
    </alarmDelay>
    </nightLongViewMultiBrightness>
    <detectType>none</detectType>
    </FloodlightTask>
    </body>
    ```

- 252: BatteryList

  - Client

    Camera Only Message.

  - Camera

    - Header

    | Magic       | Message ID  | Message Length | Encryption Offset | Status Code | Message Class | Payload Offset |
    | ----------- | ----------- | -------------- | ----------------- | ----------- | ------------- | -------------- |
    | f0 de bc 0a | fc 00 00 00 | 1c 02 00 00    | 00 00 00 00       | c8 00       | 00 00         | 00 00 00 00    |

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
        <BatteryList version="1.1">
            <BatteryInfo>
                <channelId>0</channelId>
                <chargeStatus>chargeComplete</chargeStatus>
                <adapterStatus>solarPanel</adapterStatus>
                <voltage>3999</voltage>
                <current>0</current>
                <temperature>21</temperature>
                <batteryPercent>100</batteryPercent>
                <lowPower>0</lowPower>
                <batteryVersion>2</batteryVersion>
            </BatteryInfo>
        </BatteryList>
    </body>
    ```

    **Notes**: Sent after login, with a message handle of 0 (usually 0 means not sent in reply to a specific request). TODO: Find out how to request on demand.

- 252 <BatteryList>

  - Client
    None: This is a camera event
  - Camera:

    Standard header

  - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <BatteryList version="1.1">
    <BatteryInfo>
    <channelId>0</channelId>
    <chargeStatus>charging</chargeStatus>
    <adapterStatus>solarPanel</adapterStatus>
    <voltage>4083</voltage>
    <current>-396</current>
    <temperature>32</temperature>
    <batteryPercent>100</batteryPercent>
    <lowPower>0</lowPower>
    <batteryVersion>2</batteryVersion>
    </BatteryInfo>
    </BatteryList>
    </body>
    ```

- 255 <Net3g4gInfo>

  - Client
    NONE: This is a camera event

  - Camera:
    Standard header

  - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    ```

- 295: `<StartZoomFocus>` (Write)

  - Client

    - Extension: Usual extension xml with channel

    - Payload

    ```xml
    <?xml version="1.0" encoding="UTF-8" ?>
    <body>
    <StartZoomFocus version="1.1">
    <channelId>1</ch
    ```

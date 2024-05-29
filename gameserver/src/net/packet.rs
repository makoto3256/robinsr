use anyhow::Result;
use paste::paste;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::Instrument;

use proto::*;

use super::handlers::*;
use super::PlayerSession;

const HEAD_MAGIC: u32 = 0x9D74C714;
const TAIL_MAGIC: u32 = 0xD7A152C8;

pub struct NetPacket {
    pub cmd_type: u16,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
}

impl From<NetPacket> for Vec<u8> {
    fn from(value: NetPacket) -> Self {
        let mut out = Self::new();

        out.extend(HEAD_MAGIC.to_be_bytes());
        out.extend(value.cmd_type.to_be_bytes());
        out.extend((value.head.len() as u16).to_be_bytes());
        out.extend((value.body.len() as u32).to_be_bytes());
        out.extend(value.head);
        out.extend(value.body);
        out.extend(TAIL_MAGIC.to_be_bytes());
        out
    }
}

impl NetPacket {
    pub async fn read(stream: &mut TcpStream) -> std::io::Result<Self> {
        assert_eq!(stream.read_u32().await?, HEAD_MAGIC);
        let cmd_type = stream.read_u16().await?;

        let head_length = stream.read_u16().await? as usize;
        let body_length = stream.read_u32().await? as usize;

        let mut head = vec![0; head_length];
        stream.read_exact(&mut head).await?;

        let mut body = vec![0; body_length];
        stream.read_exact(&mut body).await?;

        assert_eq!(stream.read_u32().await?, TAIL_MAGIC);

        Ok(Self {
            cmd_type,
            head,
            body,
        })
    }
}

macro_rules! trait_handler {
    ($($name:ident $cmd_type:expr;)*) => {
        pub trait CommandHandler {
            $(
                paste! {
                    async fn [<on_$name:snake>](session: &mut PlayerSession, body: &$name) -> Result<()> {
                        [<on_$name:snake>](session, body).await
                    }
                }
            )*

            async fn on_message(session: &mut PlayerSession, cmd_type: u16, payload: Vec<u8>) -> Result<()> {
                use ::prost::Message;
                if PlayerSession::should_send_dummy_rsp(cmd_type) {
                    session.send_dummy_response(cmd_type).await?;
                    return Ok(());
                }
                match cmd_type {
                    $(
                        $cmd_type => {
                            let body = $name::decode(&mut &payload[..])?;
                            paste! {
                                Self::[<on_$name:snake>](session, &body)
                                    .instrument(tracing::info_span!(stringify!([<on_$name:snake>]), cmd_type = cmd_type))
                                    .await
                            }
                        }
                    )*
                    _ => {
                        tracing::warn!("Unknown command type: {cmd_type}");
                        Ok(())
                    },
                }
            }
        }
    };
}

trait_handler! {
    PlayerGetTokenCsReq 39;         // PlayerGetTokenScRsp
    PlayerLoginCsReq 61;            // PlayerLoginScRsp, PlayerBasicInfo
    GetMissionStatusCsReq 1256;     // GetMissionStatusScRsp, Mission
    GetBasicInfoCsReq 90;           // GetBasicInfoScRsp, PlayerSettingInfo
    GetHeroBasicTypeInfoCsReq 68;   // GetHeroBasicTypeInfoScRsp, HeroBasicTypeInfo
    GetAvatarDataCsReq 361;         // GetAvatarDataScRsp, Avatar
    GetAllLineupDataCsReq 756;      // GetAllLineupDataScRsp, LineupInfo, ExtraLineupType, LineupAvatar, AmountInfo
    GetCurLineupDataCsReq 791;      // GetCurLineupDataScRsp
    GetCurSceneInfoCsReq 1430;      // GetCurSceneInfoScRsp, SceneInfo
    PlayerHeartBeatCsReq 42;        // PlayerHeartBeatScRsp

    // Tutorial (dummy!)
    GetTutorialGuideCsReq 1691;
    UnlockTutorialGuideCsReq 1630;
    GetTutorialCsReq 1661;

    // Entity move (dummy!)
    SceneEntityMoveCsReq 1461;

    // Inventory (dummy!)
    GetBagCsReq 561;
    GetArchiveDataCsReq 2361;
    DressAvatarCsReq 387;
    TakeOffEquipmentCsReq 362;
    Gfmigicacfn 321;    // DressRelicAvatarCsReq
    Nbmofdgfejk 303;    // TakeOffRelicCsReq

    // Chat (dummy!)
    Dgaiigecbee 3961;   // SendMsgCsReq
    Pignjacjgdl 3939;   // GetPrivateChatHistoryCsReq
    Jhfffmnkcbf 2961;   // GetFriendListInfoCsReq

    // In-game lineup
    JoinLineupCsReq 739;
    ChangeLineupLeaderCsReq 794;
    ReplaceLineupCsReq 709;
    QuitLineupCsReq 730;

    // Battle
    StartCocoonStageCsReq 1413;
    PveBattleResultCsReq 161;
    SceneCastSkillCsReq 1439;

    // Teleport
    GetEnteredSceneCsReq 1407;
    GetSceneMapInfoCsReq 1484;
    EnterSceneCsReq 1480;

    // Optional
    GetMailCsReq 861;
    GetGachaInfoCsReq 1961;
    DoGachaCsReq 1991;
    GetQuestDataCsReq 961;
    PlayerLoginFinishCsReq 86;
}

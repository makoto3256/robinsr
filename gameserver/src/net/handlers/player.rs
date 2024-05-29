use crate::{net::tools::JsonData, util};

use super::*;

pub async fn on_get_basic_info_cs_req(
    session: &mut PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: 0,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_hero_basic_type_info_cs_req(
    session: &mut PlayerSession,
    _body: &GetHeroBasicTypeInfoCsReq,
) -> Result<()> {
    let mc = JsonData::load().await.main_character;
    session
        .send(
            CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
            GetHeroBasicTypeInfoScRsp {
                retcode: 0,
                gender: mc.get_gender().into(),
                cur_basic_type: mc.get_type().into(),
                basic_type_info_list: vec![HeroBasicTypeInfo {
                    basic_type: mc.get_type().into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_heart_beat_cs_req(
    session: &mut PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(ClientDownloadData {
                    version: 51,
                    time: util::cur_timestamp_ms() as i64,
                    data: rbase64::decode("G0x1YVMBGZMNChoKBAQICHhWAAAAAAAAAAAAAAAod0ABKEBDOlxVc2Vyc1x4ZW9uZGV2XERvd25sb2Fkc1xyYWJzdHZvLmx1YQAAAAAAAAAAAAEEEAAAACQAQAApQEAAKYBAACnAQABWAAEALIAAAR1AQQCkgEEA5ABAAOnAwQHpAMIB6UDCAawAAAEsgAAAH8BChRkAgAAMAAAABANDUwQMVW5pdHlFbmdpbmUEC0dhbWVPYmplY3QEBUZpbmQEKVVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkEF0dldENvbXBvbmVudEluQ2hpbGRyZW4EB3R5cGVvZgQEUlBHBAdDbGllbnQEDkxvY2FsaXplZFRleHQEBXRleHQURVJvYmluU1IgaXMgYSBmcmVlIGFuZCBvcGVuIHNvdXJjZSBzb2Z0d2FyZS4gZGlzY29yZC5nZy9yZXZlcnNlZHJvb21zAQAAAAEAAAAAABAAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAQAAAAEAAAABAAAAAAAAAAEAAAAFX0VOVg==").unwrap()
                }),
            },
        )
        .await
}

pub type PlayerLoginFinishCsReq = Dummy;

pub async fn on_player_login_finish_cs_req(
    session: &mut PlayerSession,
    _: &PlayerLoginFinishCsReq,
) -> Result<()> {
    session
        .send(CMD_PLAYER_LOGIN_FINISH_SC_RSP, Dummy {})
        .await?;
    session
        .send(CMD_CONTENT_PACKAGE_UNLOCK_SC_RSP, Dummy {})
        .await?;
    session
        .send(CMD_CONTENT_PACKAGE_GET_DATA_SC_RSP, Dummy {})
        .await?;
    session
        .send(
            CMD_CONTENT_PACKAGE_SYNC_DATA_SC_NOTIFY,
            Chhopfkjmje {
                data: Some(Gdafmkkhkkl {
                    himejaheaoj: vec![Bejmehlnpan {
                        status: Olngclnnaie::ContentPackageStatusFinished.into(),
                        jkbgighlakf: 200001,
                    }],
                    eoljolnkooh: 0,
                }),
            },
        )
        .await?;

    Ok(())
}

use anyhow::Result;
use proto::*;

use crate::net::{tools::JsonData, PlayerSession};

use super::Dummy;

pub async fn on_get_bag_cs_req(session: &mut PlayerSession, _: &GetBagCsReq) -> Result<()> {
    let player = JsonData::load().await;

    session
        .send(
            CMD_GET_BAG_SC_RSP,
            GetBagScRsp {
                equipment_list: player
                    .lightcones
                    .iter()
                    .map(|v| v.to_equipment_proto())
                    .collect(),
                relic_list: player.relics.iter().map(|v| v.to_relic_proto()).collect(),
                macfjibhfad: vec![
                    Efdnhdlcegi {
                        tid: 101, // Normal Pass
                        num: 999999,
                        ..Default::default()
                    },
                    Efdnhdlcegi {
                        tid: 102, // Special Pass
                        num: 999999,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_archive_data_cs_req(
    session: &mut PlayerSession,
    _: &GetArchiveDataCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_ARCHIVE_DATA_SC_RSP,
            GetArchiveDataScRsp {
                bkcflgbcjmi: Some(ArchiveData::default()),
                retcode: 0,
            },
        )
        .await
}

pub async fn on_gfmigicacfn(session: &mut PlayerSession, _: &Gfmigicacfn) -> Result<()> {
    // ?
    session
        .send(CMD_DRESS_RELIC_AVATAR_SC_RSP, Dummy::default())
        .await
}

pub async fn on_nbmofdgfejk(session: &mut PlayerSession, _: &Nbmofdgfejk) -> Result<()> {
    // ?
    session
        .send(CMD_TAKE_OFF_RELIC_SC_RSP, Dummy::default())
        .await
}

pub async fn on_dress_avatar_cs_req(
    session: &mut PlayerSession,
    _: &DressAvatarCsReq,
) -> Result<()> {
    // ?
    session
        .send(CMD_DRESS_AVATAR_SC_RSP, Dummy::default())
        .await
}

pub async fn on_take_off_equipment_cs_req(
    session: &mut PlayerSession,
    _: &TakeOffEquipmentCsReq,
) -> Result<()> {
    // ?
    session
        .send(CMD_TAKE_OFF_EQUIPMENT_SC_RSP, Dummy::default())
        .await
}

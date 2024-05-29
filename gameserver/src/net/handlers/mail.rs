use super::*;
use anyhow::Result;

pub async fn on_get_mail_cs_req(session: &mut PlayerSession, _: &GetMailCsReq) -> Result<()> {
    session
        .send(
            CMD_GET_MAIL_SC_RSP,
            GetMailScRsp {
                is_end: true,
                mail_list: vec![ClientMail {
                    title: String::from("Welcome!"),
                    sender: String::from("Server"),
                    content: String::from("Welcome!"),
                    id: 1,
                    is_read: false,
                    time: 1716041089,
                    expire_time: 1718633089,
                    para_list: vec![],
                    attachment: Some(ItemList {
                        item_list: vec![Item {
                            item_id: 1310,
                            level: 80,
                            num: 1,
                            ..Default::default()
                        }],
                    }),
                    mail_type: MailType::Normal.into(),
                    template_id: 0,
                }],
                notice_mail_list: vec![],
                start: 0,
                retcode: 0,
                total_num: 1,
            },
        )
        .await
}
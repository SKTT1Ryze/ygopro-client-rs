//! YGOPro message protocol between client and server

pub mod traits;

#[repr(C)]
pub struct YGOPacket {
    pub packet_len: u16,
    pub proto: u8,
    pub exdata: Vec<u8>,
}

impl YGOPacket {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        todo!()
    }
    pub fn into_bytes(self) -> anyhow::Result<Vec<u8>> {
        let len = self.packet_len as usize + 2;
        let bytes = Vec::with_capacity(len);

        unsafe {
            let (ptr, _, _) = bytes.into_raw_parts();

            *(ptr as *mut u16) = self.packet_len; // write packet_len

            (ptr as *mut u8).offset(2).write(self.proto); // write proto

            (ptr as *mut u8)
                .offset(3)
                .copy_from(self.exdata.as_ptr(), self.exdata.len()); // write
                                                                     // exdata

            Ok(Vec::from_raw_parts(ptr, len, len))
        }
    }

    pub fn from_proto<T: traits::IntoExdata>(proto: YGOProto, exdata: T) -> anyhow::Result<Self> {
        match proto {
            YGOProto::CTOS(ctos) => match ctos {
                CTOSMsg::PLAYER_INFO | CTOSMsg::JOIN_GAME => {
                    let exdata = exdata.into_exdata();

                    Ok(Self {
                        packet_len: exdata.len() as u16 + 1,
                        proto: ctos as u8,
                        exdata,
                    })
                }
                _ => todo!(),
            },
            YGOProto::STOC(stoc) => todo!(),
        }
    }
}

pub enum YGOProto {
    CTOS(CTOSMsg),
    STOC(STOCMsg),
}

#[repr(u8)]
pub enum CTOSMsg {
    RESPONSE = 1,
    UPDATE_DECK = 2,
    HAND_RESULT = 3,
    TP_RESULT = 4,
    PLAYER_INFO = 16,
    CREATE_GAME = 17,
    JOIN_GAME = 18,
    LEAVE_GAME = 19,
    SURRENDER = 20,
    TIME_CONFIRM = 21,
    CHAT = 22,
    HS_TODUELIST = 32,
    HS_TOOBSERVER = 33,
    HS_READY = 34,
    HS_NOTREADY = 35,
    HS_KICK = 36,
    HS_START = 37,
    REQUEST_FIELD = 48,
}

#[repr(u8)]
pub enum STOCMsg {
    GAME_MSG = 1,
    ERROR_MSG = 2,
    SELECT_HAND = 3,
    SELECT_TP = 4,
    HAND_RESULT = 5,
    TP_RESULT = 6,
    CHANGE_SIDE = 7,
    WAITTING_SIDE = 8,
    DECK_COUNT = 9,
    CREATE_GAME = 17,
    JOIN_GAME = 18,
    TYPE_CHANGE = 19,
    LEAVE_GAME = 20,
    DUEL_START = 21,
    DUEL_END = 22,
    REPLAY = 23,
    TIME_LIMIT = 24,
    CHAT = 25,
    HS_PLAYER_ENTER = 32,
    HS_PLAYER_CHANGE = 33,
    HS_WATCH_CHANGE = 34,
    FIELD_FINISH = 48,
}
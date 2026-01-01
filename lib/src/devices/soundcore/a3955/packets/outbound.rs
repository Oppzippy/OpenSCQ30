use crate::devices::soundcore::{a3955::structures::AncPersonalizedToEarCanal, common::packet};

pub fn set_anc_personalized_to_hear_canal(
    anc_personalized_to_ear_canal: &AncPersonalizedToEarCanal,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([3, 144]),
        anc_personalized_to_ear_canal.bytes().to_vec(),
    )
}

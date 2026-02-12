use crate::devices::soundcore::{
    a3957::structures::{AncPersonalizedToEarCanal, ImmersiveExperience},
    common::packet,
};

pub fn set_anc_personalized_to_hear_canal(
    anc_personalized_to_ear_canal: &AncPersonalizedToEarCanal,
) -> packet::Outbound {
    packet::Outbound::new(
        packet::Command([3, 144]),
        anc_personalized_to_ear_canal.bytes().to_vec(),
    )
}

pub fn set_immersive_experience(immersive_experience: ImmersiveExperience) -> packet::Outbound {
    packet::Outbound::new(packet::Command([18, 129]), vec![immersive_experience as u8])
}

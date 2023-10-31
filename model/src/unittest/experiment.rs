use byteserde_derive::*;
use byteserde_types::string_ascii_fixed;

string_ascii_fixed!(
    UserName1,
    6,
    b' ',
    true,
    derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)
);

#[cfg(test)]
mod test {
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{from_str, to_string};

    // use crate::unittest::experiment::{Shell, UserName1};
    use crate::unittest::experiment::UserName1;

    // use super::Experiment;
    #[test]
    fn test_experiment0() {
        setup::log::configure();
        let msg_inp: UserName1 = b"aaa     ".as_slice().into();
        info!("msg_inp: {:?}", msg_inp);
        let json = to_string(&msg_inp).unwrap();
        info!("json: {}", json);

        let msg_out: UserName1 = from_str(&json).unwrap();
        info!("msg_out:? {:?}", msg_out);
        let inp = r#" "aaa   " "#.to_owned();
        // assert_eq!(json, inp);
        let msg_out = from_str::<UserName1>(&inp).unwrap();
        info!("msg_out:? {:?}", msg_out)
    }
    // #[test]
    // fn test_experiment() {
    //     setup::log::configure();
    //     let msg_inp: Experiment = b"aaa     ".as_slice().into();
    //     let json = to_string(&msg_inp).unwrap();
    //     info!("json: {}", json);

    //     let msg_out: Experiment = from_str(&json).unwrap();
    //     info!("msg_out:? {:?}", msg_out);
    // }
    // #[test]
    // fn test_shell() {
    //     setup::log::configure();
    //     let exp1: Experiment = b"aaa     ".as_slice().into();
    //     let exp2: Experiment = b"bbb     ".as_slice().into();
    //     let msg_inp = Shell { exp1, exp2 };
    //     let json = to_string(&msg_inp).unwrap();
    //     info!("json: {}", json);

    //     let msg_out: Shell = from_str(&json).unwrap();
    //     info!("msg_out:? {:?}", msg_out);
    // }
}

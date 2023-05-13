use std::collections::BTreeSet;

use sb_block_def_genie::*;

trait ShorterIntoString {
    fn s(self) -> String;
}

impl<T: Into<String>> ShorterIntoString for T {
    fn s(self) -> String {
        self.into()
    }
}

fn main() {
    let blocks = Blocks(vec![
        Block {
            name: "motion_setx".s(),
            identifier: "set_x".s(),
            doc: "set position in x axis to a value".s(),
            block_type: BlockType::Stack,
            parameters: vec![Parameter {
                parameter_type: ParameterType::Number,
                identifier: "x".s(),
                key: "X".s(),
                ..Default::default()
            }
            .into()],
            ..Default::default()
        },
        Block {
            identifier: "pick_random".s(),
            name: "operator_random".s(),
            block_type: BlockType::Reporter,
            doc: "".s(),
            parameters: vec![
                Parameter {
                    parameter_type: ParameterType::Number,
                    identifier: "from".s(),
                    key: "FROM".s(),
                    ..Default::default()
                }
                .into(),
                "to".s().into(),
                Parameter {
                    parameter_type: ParameterType::Number,
                    identifier: "to".s(),
                    key: "TO".s(),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        },
        Block {
            identifier: "touching_menu".s(),
            name: "sensing_touchingobjectmenu".s(),
            doc: "".s(),
            block_type: BlockType::Menu,
            parameters: vec![Parameter {
                identifier: "object".s(),
                key: "TOUCHINGOBJECTMENU".s(),
                parameter_type: ParameterType::Field {
                    possible_values: Some(BTreeSet::from_iter(["_mouse_".s(), "_edge".s()])),
                    possible_categories: Some(BTreeSet::from_iter(["sprite_name".s()])),
                },
                ..Default::default()
            }
            .into()],
            ..Default::default()
        },
        Block {
            identifier: "touching".s(),
            name: "sensing_touchingobject".s(),
            doc: "".s(),
            block_type: BlockType::Reporter,
            parameters: vec![
                Parameter {
                    identifier: "object".s(),
                    key: "TOUCHINGOBJECTMENU".s(),
                    parameter_type: ParameterType::UnrestrictedField {
                        menu_block: "sensing_touchingobjectmenu".s(),
                    },
                    ..Default::default()
                }
                .into(),
                "?".s().into(),
            ],
            ..Default::default()
        },
    ]);
    let serialized = blocks.to_string().unwrap();
    let serialized = serialized.replace("\n- display_name", "\n\n- display_name");
    println!("{serialized}");
}

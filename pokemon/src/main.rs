mod nonterminals;
#[derive(Debug)]
struct _S;
impl<'a> wagon_gll::Label<'a> for _S {
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("S") ?], None)])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        wagon_gll::ROOT_UUID
    }
    fn to_string(&self) -> &str {
        wagon_gll::ROOT_UUID
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![wagon_gll::ROOT_UUID]
    }
    fn code(&self, _: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        unreachable!("This should never be called");
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (Vec::new(), Vec::new())
    }
    fn _weight(
        &self,
        _state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        unreachable!("This should never be called");
    }
}
#[allow(non_snake_case, unused_mut)]
fn main() {
    let args = clap::command!()
        .arg(
            clap::arg!(< filename > "Input file to parse")
                .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
        .arg(clap::arg!(- - "no-crop" "Don't crop resulting sppf").num_args(0))
        .arg(
            clap::arg!(
                - - "math-mode" "Print SPPF dot labels in Latex math-mode representation"
            )
                .num_args(0),
        )
        .arg(
            clap::arg!(- - "print-gss" "Also print the final GSS (works with math-mode)")
                .num_args(0),
        )
        .get_matches();
    let input_file = args
        .get_one::<std::path::PathBuf>("filename")
        .expect("Input file required");
    let input_file_str = Box::leak(input_file.to_str().unwrap().into());
    let crop = args.get_one::<bool>("no-crop").unwrap_or(&false) == &false;
    let math_mode = *args.get_one::<bool>("math-mode").unwrap_or(&false);
    let print_gss = *args.get_one::<bool>("print-gss").unwrap_or(&false);
    let content_string = std::fs::read_to_string(input_file)
        .expect("Couldn't read file");
    let contents: &'static [u8] = Box::leak(content_string.trim().as_bytes().into());
    let mut label_map: wagon_gll::LabelMap = std::collections::HashMap::with_capacity(
        196usize,
    );
    let mut rule_map: wagon_gll::RuleMap = std::collections::HashMap::with_capacity(
        41usize,
    );
    let mut regex_map: wagon_gll::RegexMap = std::collections::HashMap::with_capacity(
        1usize,
    );
    let label_0 = std::rc::Rc::new(nonterminals::NumberList::NumberList {
    });
    label_map.insert("NumberList", label_0);
    let alt_NumberList_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("NumberList_0_0".to_string()),
            wagon_ident::Ident::Unknown("NumberList_0_1".to_string()),
            wagon_ident::Ident::Unknown("NumberList_0_2".to_string()),
        ],
    );
    rule_map.insert("NumberList_0", alt_NumberList_0);
    let alt_NumberList_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("NumberList_1_0".to_string()),
            wagon_ident::Ident::Unknown("NumberList_1_1".to_string()),
        ],
    );
    rule_map.insert("NumberList_1", alt_NumberList_1);
    let label_1 = std::rc::Rc::new(nonterminals::Ability::Ability {});
    label_map.insert("Ability", label_1);
    let alt_Ability_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Ability_0_0".to_string()),
            wagon_ident::Ident::Unknown("Ability_0_1".to_string()),
        ],
    );
    rule_map.insert("Ability_0", alt_Ability_0);
    let label_2 = std::rc::Rc::new(nonterminals::Happiness::Happiness_0_2::Happiness_0_2 {});
    label_map.insert("Happiness_0_2", label_2);
    let label_3 = std::rc::Rc::new(nonterminals::IVs::IVs {});
    label_map.insert("IVs", label_3);
    let alt_IVs_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("IVs_0_0".to_string()),
            wagon_ident::Ident::Unknown("IVs_0_1".to_string()),
            wagon_ident::Ident::Unknown("IVs_0_2".to_string()),
            wagon_ident::Ident::Unknown("IVs_0_3".to_string()),
        ],
    );
    rule_map.insert("IVs_0", alt_IVs_0);
    let label_4 = std::rc::Rc::new(nonterminals::Info·0·2··0::Info·0·2··0_0_1::Info·0·2··0_0_1 {});
    label_map.insert("Info·0·2··0_0_1", label_4);
    let label_5 = std::rc::Rc::new(nonterminals::Number::Number_2_0::Number_2_0 {
    });
    label_map.insert("Number_2_0", label_5);
    let label_6 = std::rc::Rc::new(nonterminals::Pokemon·0·4::Pokemon·0·4_0_2::Pokemon·0·4_0_2 {});
    label_map.insert("Pokemon·0·4_0_2", label_6);
    let label_7 = std::rc::Rc::new(nonterminals::Optionals::Optionals_2_1::Optionals_2_1 {});
    label_map.insert("Optionals_2_1", label_7);
    let label_8 = std::rc::Rc::new(nonterminals::Optionals::Optionals_5_1::Optionals_5_1 {});
    label_map.insert("Optionals_5_1", label_8);
    let label_9 = std::rc::Rc::new(nonterminals::Ability::Ability_0_0::Ability_0_0 {
    });
    label_map.insert("Ability_0_0", label_9);
    let label_10 = std::rc::Rc::new(nonterminals::MoveList::MoveList_1_0::MoveList_1_0 {
    });
    label_map.insert("MoveList_1_0", label_10);
    let label_11 = std::rc::Rc::new(nonterminals::Number::Number_4_0::Number_4_0 {
    });
    label_map.insert("Number_4_0", label_11);
    let label_12 = std::rc::Rc::new(nonterminals::S::S_0_1::S_0_1 {});
    label_map.insert("S_0_1", label_12);
    let label_13 = std::rc::Rc::new(nonterminals::Level::Level_0_1::Level_0_1 {
    });
    label_map.insert("Level_0_1", label_13);
    let label_14 = std::rc::Rc::new(nonterminals::Nature::Nature_4_0::Nature_4_0 {
    });
    label_map.insert("Nature_4_0", label_14);
    let label_15 = std::rc::Rc::new(nonterminals::Nature::Nature_5_0::Nature_5_0 {
    });
    label_map.insert("Nature_5_0", label_15);
    let label_16 = std::rc::Rc::new(nonterminals::Nature::Nature_9_0::Nature_9_0 {
    });
    label_map.insert("Nature_9_0", label_16);
    let label_17 = std::rc::Rc::new(nonterminals::EVStart::EVStart_0_1::EVStart_0_1 {
    });
    label_map.insert("EVStart_0_1", label_17);
    let label_18 = std::rc::Rc::new(nonterminals::EVGuard::EVGuard_0_0::EVGuard_0_0 {
    });
    label_map.insert("EVGuard_0_0", label_18);
    let label_19 = std::rc::Rc::new(nonterminals::Nature::Nature {});
    label_map.insert("Nature", label_19);
    let alt_Nature_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_0_0".to_string()),],
    );
    rule_map.insert("Nature_0", alt_Nature_0);
    let alt_Nature_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_1_0".to_string()),],
    );
    rule_map.insert("Nature_1", alt_Nature_1);
    let alt_Nature_2 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_2_0".to_string()),],
    );
    rule_map.insert("Nature_2", alt_Nature_2);
    let alt_Nature_3 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_3_0".to_string()),],
    );
    rule_map.insert("Nature_3", alt_Nature_3);
    let alt_Nature_4 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_4_0".to_string()),],
    );
    rule_map.insert("Nature_4", alt_Nature_4);
    let alt_Nature_5 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_5_0".to_string()),],
    );
    rule_map.insert("Nature_5", alt_Nature_5);
    let alt_Nature_6 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_6_0".to_string()),],
    );
    rule_map.insert("Nature_6", alt_Nature_6);
    let alt_Nature_7 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_7_0".to_string()),],
    );
    rule_map.insert("Nature_7", alt_Nature_7);
    let alt_Nature_8 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_8_0".to_string()),],
    );
    rule_map.insert("Nature_8", alt_Nature_8);
    let alt_Nature_9 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_9_0".to_string()),],
    );
    rule_map.insert("Nature_9", alt_Nature_9);
    let alt_Nature_10 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_10_0".to_string()),],
    );
    rule_map.insert("Nature_10", alt_Nature_10);
    let alt_Nature_11 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_11_0".to_string()),],
    );
    rule_map.insert("Nature_11", alt_Nature_11);
    let alt_Nature_12 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_12_0".to_string()),],
    );
    rule_map.insert("Nature_12", alt_Nature_12);
    let alt_Nature_13 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_13_0".to_string()),],
    );
    rule_map.insert("Nature_13", alt_Nature_13);
    let alt_Nature_14 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_14_0".to_string()),],
    );
    rule_map.insert("Nature_14", alt_Nature_14);
    let alt_Nature_15 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_15_0".to_string()),],
    );
    rule_map.insert("Nature_15", alt_Nature_15);
    let alt_Nature_16 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_16_0".to_string()),],
    );
    rule_map.insert("Nature_16", alt_Nature_16);
    let alt_Nature_17 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_17_0".to_string()),],
    );
    rule_map.insert("Nature_17", alt_Nature_17);
    let alt_Nature_18 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_18_0".to_string()),],
    );
    rule_map.insert("Nature_18", alt_Nature_18);
    let alt_Nature_19 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_19_0".to_string()),],
    );
    rule_map.insert("Nature_19", alt_Nature_19);
    let alt_Nature_20 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_20_0".to_string()),],
    );
    rule_map.insert("Nature_20", alt_Nature_20);
    let alt_Nature_21 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_21_0".to_string()),],
    );
    rule_map.insert("Nature_21", alt_Nature_21);
    let alt_Nature_22 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_22_0".to_string()),],
    );
    rule_map.insert("Nature_22", alt_Nature_22);
    let alt_Nature_23 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_23_0".to_string()),],
    );
    rule_map.insert("Nature_23", alt_Nature_23);
    let alt_Nature_24 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Nature_24_0".to_string()),],
    );
    rule_map.insert("Nature_24", alt_Nature_24);
    let label_20 = std::rc::Rc::new(nonterminals::Nature::Nature_17_0::Nature_17_0 {
    });
    label_map.insert("Nature_17_0", label_20);
    let label_21 = std::rc::Rc::new(nonterminals::AnyString::AnyString {
    });
    label_map.insert("AnyString", label_21);
    let alt_AnyString_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("AnyString_0_0".to_string()),],
    );
    rule_map.insert("AnyString_0", alt_AnyString_0);
    let label_22 = std::rc::Rc::new(nonterminals::Moves::Moves {});
    label_map.insert("Moves", label_22);
    let alt_Moves_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Moves_0_0".to_string()),
            wagon_ident::Ident::Unknown("Moves_0_1".to_string()),
        ],
    );
    rule_map.insert("Moves_0", alt_Moves_0);
    let label_23 = std::rc::Rc::new(nonterminals::Nature::Nature_24_0::Nature_24_0 {
    });
    label_map.insert("Nature_24_0", label_23);
    let label_24 = std::rc::Rc::new(nonterminals::IVs::IVs_0_2::IVs_0_2 {
    });
    label_map.insert("IVs_0_2", label_24);
    let label_25 = std::rc::Rc::new(nonterminals::IVList::IVList {});
    label_map.insert("IVList", label_25);
    let alt_IVList_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("IVList_0_0".to_string()),
            wagon_ident::Ident::Unknown("IVList_0_1".to_string()),
            wagon_ident::Ident::Unknown("IVList_0_2".to_string()),
        ],
    );
    rule_map.insert("IVList_0", alt_IVList_0);
    let alt_IVList_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("IVList_1_0".to_string()),
            wagon_ident::Ident::Unknown("IVList_1_1".to_string()),
        ],
    );
    rule_map.insert("IVList_1", alt_IVList_1);
    let label_26 = std::rc::Rc::new(nonterminals::Info::Info_0_1::Info_0_1 {
    });
    label_map.insert("Info_0_1", label_26);
    let label_27 = std::rc::Rc::new(nonterminals::Info::Info_0_2::Info_0_2 {
    });
    label_map.insert("Info_0_2", label_27);
    let label_28 = std::rc::Rc::new(nonterminals::Optionals::Optionals_6_2::Optionals_6_2 {});
    label_map.insert("Optionals_6_2", label_28);
    let label_29 = std::rc::Rc::new(nonterminals::Nature::Nature_11_0::Nature_11_0 {
    });
    label_map.insert("Nature_11_0", label_29);
    let label_30 = std::rc::Rc::new(nonterminals::Nature::Nature_13_0::Nature_13_0 {
    });
    label_map.insert("Nature_13_0", label_30);
    let label_31 = std::rc::Rc::new(nonterminals::Pokemon·0·4::Pokemon·0·4 {
    });
    label_map.insert("Pokemon·0·4", label_31);
    let alt_Pokemon·0·4_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Pokemon·0·4_0_0".to_string()),
            wagon_ident::Ident::Unknown("Pokemon·0·4_0_1".to_string()),
            wagon_ident::Ident::Unknown("Pokemon·0·4_0_2".to_string()),
        ],
    );
    rule_map.insert("Pokemon·0·4_0", alt_Pokemon·0·4_0);
    let label_32 = std::rc::Rc::new(nonterminals::Item::Item_0_0::Item_0_0 {
    });
    label_map.insert("Item_0_0", label_32);
    let label_33 = std::rc::Rc::new(nonterminals::S::S {});
    label_map.insert("S", label_33);
    let alt_S_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("S_0_0".to_string()),
            wagon_ident::Ident::Unknown("S_0_1".to_string()),
        ],
    );
    rule_map.insert("S_0", alt_S_0);
    label_map.insert(wagon_gll::ROOT_UUID, std::rc::Rc::new(_S {}));
    rule_map
        .insert(
            wagon_gll::ROOT_UUID,
            std::rc::Rc::new(vec![wagon_ident::Ident::Unknown("S".to_string())]),
        );
    let label_34 = std::rc::Rc::new(nonterminals::Nature::Nature_23_0::Nature_23_0 {
    });
    label_map.insert("Nature_23_0", label_34);
    let label_35 = std::rc::Rc::new(nonterminals::IVList::IVList_0_0::IVList_0_0 {
    });
    label_map.insert("IVList_0_0", label_35);
    let label_36 = std::rc::Rc::new(nonterminals::Name::Name_0_0::Name_0_0 {
    });
    label_map.insert("Name_0_0", label_36);
    let label_37 = std::rc::Rc::new(nonterminals::Moves::Moves_0_1::Moves_0_1 {
    });
    label_map.insert("Moves_0_1", label_37);
    let label_38 = std::rc::Rc::new(nonterminals::MoveList::MoveList {
    });
    label_map.insert("MoveList", label_38);
    let alt_MoveList_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("MoveList_0_0".to_string()),
            wagon_ident::Ident::Unknown("MoveList_0_1".to_string()),
            wagon_ident::Ident::Unknown("MoveList_0_2".to_string()),
        ],
    );
    rule_map.insert("MoveList_0", alt_MoveList_0);
    let alt_MoveList_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("MoveList_1_0".to_string()),
            wagon_ident::Ident::Unknown("MoveList_1_1".to_string()),
        ],
    );
    rule_map.insert("MoveList_1", alt_MoveList_1);
    let label_39 = std::rc::Rc::new(nonterminals::Info·0·1::Info·0·1_0_1::Info·0·1_0_1 {});
    label_map.insert("Info·0·1_0_1", label_39);
    let label_40 = std::rc::Rc::new(nonterminals::Optionals::Optionals_2_0::Optionals_2_0 {});
    label_map.insert("Optionals_2_0", label_40);
    let label_41 = std::rc::Rc::new(nonterminals::Info·0·2::Info·0·2_0_1::Info·0·2_0_1 {});
    label_map.insert("Info·0·2_0_1", label_41);
    let label_42 = std::rc::Rc::new(nonterminals::IVStart::IVStart_0_1::IVStart_0_1 {
    });
    label_map.insert("IVStart_0_1", label_42);
    let label_43 = std::rc::Rc::new(nonterminals::EVList::EVList_0_1::EVList_0_1 {
    });
    label_map.insert("EVList_0_1", label_43);
    let label_44 = std::rc::Rc::new(nonterminals::EVs::EVs {});
    label_map.insert("EVs", label_44);
    let alt_EVs_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("EVs_0_0".to_string()),
            wagon_ident::Ident::Unknown("EVs_0_1".to_string()),
            wagon_ident::Ident::Unknown("EVs_0_2".to_string()),
            wagon_ident::Ident::Unknown("EVs_0_3".to_string()),
        ],
    );
    rule_map.insert("EVs_0", alt_EVs_0);
    let label_45 = std::rc::Rc::new(nonterminals::Pokemon::Pokemon {});
    label_map.insert("Pokemon", label_45);
    let alt_Pokemon_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Pokemon_0_0".to_string()),
            wagon_ident::Ident::Unknown("Pokemon_0_1".to_string()),
            wagon_ident::Ident::Unknown("Pokemon_0_2".to_string()),
            wagon_ident::Ident::Unknown("Pokemon_0_3".to_string()),
        ],
    );
    rule_map.insert("Pokemon_0", alt_Pokemon_0);
    let label_46 = std::rc::Rc::new(nonterminals::HappyGuard::HappyGuard_0_0::HappyGuard_0_0 {});
    label_map.insert("HappyGuard_0_0", label_46);
    let label_47 = std::rc::Rc::new(nonterminals::Pokemon·0·4::Pokemon·0·4_0_0::Pokemon·0·4_0_0 {});
    label_map.insert("Pokemon·0·4_0_0", label_47);
    let label_48 = std::rc::Rc::new(nonterminals::Info·0·2::Info·0·2_0_0::Info·0·2_0_0 {});
    label_map.insert("Info·0·2_0_0", label_48);
    let label_49 = std::rc::Rc::new(nonterminals::Name::Name_0_1::Name_0_1 {
    });
    label_map.insert("Name_0_1", label_49);
    let label_50 = std::rc::Rc::new(nonterminals::Optionals::Optionals {
    });
    label_map.insert("Optionals", label_50);
    let alt_Optionals_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_0_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_0_1".to_string()),
        ],
    );
    rule_map.insert("Optionals_0", alt_Optionals_0);
    let alt_Optionals_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_1_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_1_1".to_string()),
        ],
    );
    rule_map.insert("Optionals_1", alt_Optionals_1);
    let alt_Optionals_2 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_2_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_2_1".to_string()),
        ],
    );
    rule_map.insert("Optionals_2", alt_Optionals_2);
    let alt_Optionals_3 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_3_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_3_1".to_string()),
        ],
    );
    rule_map.insert("Optionals_3", alt_Optionals_3);
    let alt_Optionals_4 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_4_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_4_1".to_string()),
        ],
    );
    rule_map.insert("Optionals_4", alt_Optionals_4);
    let alt_Optionals_5 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_5_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_5_1".to_string()),
        ],
    );
    rule_map.insert("Optionals_5", alt_Optionals_5);
    let alt_Optionals_6 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Optionals_6_0".to_string()),
            wagon_ident::Ident::Unknown("Optionals_6_1".to_string()),
            wagon_ident::Ident::Unknown("Optionals_6_2".to_string()),
        ],
    );
    rule_map.insert("Optionals_6", alt_Optionals_6);
    let label_51 = std::rc::Rc::new(nonterminals::IVList::IVList_0_1::IVList_0_1 {
    });
    label_map.insert("IVList_0_1", label_51);
    let label_52 = std::rc::Rc::new(nonterminals::IVList::IVList_1_0::IVList_1_0 {
    });
    label_map.insert("IVList_1_0", label_52);
    let label_53 = std::rc::Rc::new(nonterminals::IVs::IVs_0_0::IVs_0_0 {
    });
    label_map.insert("IVs_0_0", label_53);
    let label_54 = std::rc::Rc::new(nonterminals::IVList::IVList_1_1::IVList_1_1 {
    });
    label_map.insert("IVList_1_1", label_54);
    let label_55 = std::rc::Rc::new(nonterminals::Number::Number_0_0::Number_0_0 {
    });
    label_map.insert("Number_0_0", label_55);
    let label_56 = std::rc::Rc::new(nonterminals::Happiness::Happiness_0_1::Happiness_0_1 {});
    label_map.insert("Happiness_0_1", label_56);
    let label_57 = std::rc::Rc::new(nonterminals::EVs::EVs_0_1::EVs_0_1 {
    });
    label_map.insert("EVs_0_1", label_57);
    let label_58 = std::rc::Rc::new(nonterminals::EVList::EVList_1_1::EVList_1_1 {
    });
    label_map.insert("EVList_1_1", label_58);
    let label_59 = std::rc::Rc::new(nonterminals::Number::Number_3_0::Number_3_0 {
    });
    label_map.insert("Number_3_0", label_59);
    let label_60 = std::rc::Rc::new(nonterminals::Nature::Nature_12_0::Nature_12_0 {
    });
    label_map.insert("Nature_12_0", label_60);
    let label_61 = std::rc::Rc::new(nonterminals::Nature::Nature_21_0::Nature_21_0 {
    });
    label_map.insert("Nature_21_0", label_61);
    let label_62 = std::rc::Rc::new(nonterminals::Optionals::Optionals_3_0::Optionals_3_0 {});
    label_map.insert("Optionals_3_0", label_62);
    let label_63 = std::rc::Rc::new(nonterminals::Pokemon::Pokemon_0_1::Pokemon_0_1 {
    });
    label_map.insert("Pokemon_0_1", label_63);
    let label_64 = std::rc::Rc::new(nonterminals::Level::Level {});
    label_map.insert("Level", label_64);
    let alt_Level_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Level_0_0".to_string()),
            wagon_ident::Ident::Unknown("Level_0_1".to_string()),
            wagon_ident::Ident::Unknown("Level_0_2".to_string()),
        ],
    );
    rule_map.insert("Level_0", alt_Level_0);
    let label_65 = std::rc::Rc::new(nonterminals::Stat::Stat_4_0::Stat_4_0 {
    });
    label_map.insert("Stat_4_0", label_65);
    let label_66 = std::rc::Rc::new(nonterminals::Optionals::Optionals_6_0::Optionals_6_0 {});
    label_map.insert("Optionals_6_0", label_66);
    let label_67 = std::rc::Rc::new(nonterminals::IVList::IVList_0_2::IVList_0_2 {
    });
    label_map.insert("IVList_0_2", label_67);
    let label_68 = std::rc::Rc::new(nonterminals::Nature::Nature_19_0::Nature_19_0 {
    });
    label_map.insert("Nature_19_0", label_68);
    let label_69 = std::rc::Rc::new(nonterminals::Info::Info_0_0::Info_0_0 {
    });
    label_map.insert("Info_0_0", label_69);
    let label_70 = std::rc::Rc::new(nonterminals::Stat::Stat_0_0::Stat_0_0 {
    });
    label_map.insert("Stat_0_0", label_70);
    let label_71 = std::rc::Rc::new(nonterminals::Stat::Stat_2_0::Stat_2_0 {
    });
    label_map.insert("Stat_2_0", label_71);
    let label_72 = std::rc::Rc::new(nonterminals::Optionals::Optionals_4_0::Optionals_4_0 {});
    label_map.insert("Optionals_4_0", label_72);
    let label_73 = std::rc::Rc::new(nonterminals::Stat::Stat_5_0::Stat_5_0 {
    });
    label_map.insert("Stat_5_0", label_73);
    let label_74 = std::rc::Rc::new(nonterminals::Pokemon·0·2::Pokemon·0·2 {
    });
    label_map.insert("Pokemon·0·2", label_74);
    let alt_Pokemon·0·2_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Pokemon·0·2_0_0".to_string()),
            wagon_ident::Ident::Unknown("Pokemon·0·2_0_1".to_string()),
            wagon_ident::Ident::Unknown("Pokemon·0·2_0_2".to_string()),
        ],
    );
    rule_map.insert("Pokemon·0·2_0", alt_Pokemon·0·2_0);
    let alt_Pokemon·0·2_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Pokemon·0·2_1_0".to_string()),],
    );
    rule_map.insert("Pokemon·0·2_1", alt_Pokemon·0·2_1);
    let label_75 = std::rc::Rc::new(nonterminals::Gender::Gender {});
    label_map.insert("Gender", label_75);
    let alt_Gender_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Gender_0_0".to_string()),],
    );
    rule_map.insert("Gender_0", alt_Gender_0);
    let alt_Gender_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Gender_1_0".to_string()),],
    );
    rule_map.insert("Gender_1", alt_Gender_1);
    let label_76 = std::rc::Rc::new(nonterminals::Gender::Gender_1_0::Gender_1_0 {
    });
    label_map.insert("Gender_1_0", label_76);
    let label_77 = std::rc::Rc::new(nonterminals::NatureDef::NatureDef_0_0::NatureDef_0_0 {});
    label_map.insert("NatureDef_0_0", label_77);
    let label_78 = std::rc::Rc::new(nonterminals::Level::Level_0_2::Level_0_2 {
    });
    label_map.insert("Level_0_2", label_78);
    let label_79 = std::rc::Rc::new(nonterminals::Happiness::Happiness {
    });
    label_map.insert("Happiness", label_79);
    let alt_Happiness_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Happiness_0_0".to_string()),
            wagon_ident::Ident::Unknown("Happiness_0_1".to_string()),
            wagon_ident::Ident::Unknown("Happiness_0_2".to_string()),
        ],
    );
    rule_map.insert("Happiness_0", alt_Happiness_0);
    let label_80 = std::rc::Rc::new(nonterminals::Stat::Stat_3_0::Stat_3_0 {
    });
    label_map.insert("Stat_3_0", label_80);
    let label_81 = std::rc::Rc::new(nonterminals::S::S_0_0::S_0_0 {});
    label_map.insert("S_0_0", label_81);
    let label_82 = std::rc::Rc::new(nonterminals::Pokemon·0·2::Pokemon·0·2_0_1::Pokemon·0·2_0_1 {});
    label_map.insert("Pokemon·0·2_0_1", label_82);
    let label_83 = std::rc::Rc::new(nonterminals::Info·0·1::Info·0·1_0_0::Info·0·1_0_0 {});
    label_map.insert("Info·0·1_0_0", label_83);
    let label_84 = std::rc::Rc::new(nonterminals::Moves::Moves_0_0::Moves_0_0 {
    });
    label_map.insert("Moves_0_0", label_84);
    let label_85 = std::rc::Rc::new(nonterminals::Info·0·2::Info·0·2 {
    });
    label_map.insert("Info·0·2", label_85);
    let alt_Info·0·2_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info·0·2_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info·0·2_0_1".to_string()),
        ],
    );
    rule_map.insert("Info·0·2_0", alt_Info·0·2_0);
    let alt_Info·0·2_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Info·0·2_1_0".to_string()),],
    );
    rule_map.insert("Info·0·2_1", alt_Info·0·2_1);
    let label_86 = std::rc::Rc::new(nonterminals::PerEVGuard::PerEVGuard_0_0::PerEVGuard_0_0 {});
    label_map.insert("PerEVGuard_0_0", label_86);
    let label_87 = std::rc::Rc::new(nonterminals::Pokemon::Pokemon_0_3::Pokemon_0_3 {
    });
    label_map.insert("Pokemon_0_3", label_87);
    let label_88 = std::rc::Rc::new(nonterminals::Pokemon·0·2::Pokemon·0·2_0_0::Pokemon·0·2_0_0 {});
    label_map.insert("Pokemon·0·2_0_0", label_88);
    let label_89 = std::rc::Rc::new(nonterminals::Decimal::Decimal {});
    label_map.insert("Decimal", label_89);
    let alt_Decimal_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Decimal_0_0".to_string()),
            wagon_ident::Ident::Unknown("Decimal_0_1".to_string()),
        ],
    );
    rule_map.insert("Decimal_0", alt_Decimal_0);
    let label_90 = std::rc::Rc::new(nonterminals::NumberList::NumberList_0_1::NumberList_0_1 {});
    label_map.insert("NumberList_0_1", label_90);
    let label_91 = std::rc::Rc::new(nonterminals::NumberList::NumberList_0_2::NumberList_0_2 {});
    label_map.insert("NumberList_0_2", label_91);
    let label_92 = std::rc::Rc::new(nonterminals::Number::Number_6_0::Number_6_0 {
    });
    label_map.insert("Number_6_0", label_92);
    let label_93 = std::rc::Rc::new(nonterminals::Optionals::Optionals_5_0::Optionals_5_0 {});
    label_map.insert("Optionals_5_0", label_93);
    let label_94 = std::rc::Rc::new(nonterminals::NatureDef::NatureDef {
    });
    label_map.insert("NatureDef", label_94);
    let alt_NatureDef_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("NatureDef_0_0".to_string()),
            wagon_ident::Ident::Unknown("NatureDef_0_1".to_string()),
        ],
    );
    rule_map.insert("NatureDef_0", alt_NatureDef_0);
    let label_95 = std::rc::Rc::new(nonterminals::Nature::Nature_0_0::Nature_0_0 {
    });
    label_map.insert("Nature_0_0", label_95);
    let label_96 = std::rc::Rc::new(nonterminals::Pokemon·0·4·p::Pokemon·0·4·p_0_1::Pokemon·0·4·p_0_1 {});
    label_map.insert("Pokemon·0·4·p_0_1", label_96);
    let label_97 = std::rc::Rc::new(nonterminals::Info·0·3··0::Info·0·3··0_0_0::Info·0·3··0_0_0 {});
    label_map.insert("Info·0·3··0_0_0", label_97);
    let label_98 = std::rc::Rc::new(nonterminals::MoveList::MoveList_0_2::MoveList_0_2 {
    });
    label_map.insert("MoveList_0_2", label_98);
    let label_99 = std::rc::Rc::new(nonterminals::YesNo::YesNo_1_0::YesNo_1_0 {
    });
    label_map.insert("YesNo_1_0", label_99);
    let label_100 = std::rc::Rc::new(nonterminals::IVStart::IVStart {});
    label_map.insert("IVStart", label_100);
    let alt_IVStart_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("IVStart_0_0".to_string()),
            wagon_ident::Ident::Unknown("IVStart_0_1".to_string()),
        ],
    );
    rule_map.insert("IVStart_0", alt_IVStart_0);
    let label_101 = std::rc::Rc::new(nonterminals::Info·0·1::Info·0·1_1_0::Info·0·1_1_0 {});
    label_map.insert("Info·0·1_1_0", label_101);
    let label_102 = std::rc::Rc::new(nonterminals::Optionals::Optionals_3_1::Optionals_3_1 {});
    label_map.insert("Optionals_3_1", label_102);
    let label_103 = std::rc::Rc::new(nonterminals::IVGuard::IVGuard_0_0::IVGuard_0_0 {
    });
    label_map.insert("IVGuard_0_0", label_103);
    let label_104 = std::rc::Rc::new(nonterminals::Pokemon·0·2::Pokemon·0·2_1_0::Pokemon·0·2_1_0 {});
    label_map.insert("Pokemon·0·2_1_0", label_104);
    let label_105 = std::rc::Rc::new(nonterminals::Info·0·2··0::Info·0·2··0_0_0::Info·0·2··0_0_0 {});
    label_map.insert("Info·0·2··0_0_0", label_105);
    let label_106 = std::rc::Rc::new(nonterminals::MoveList::MoveList_1_1::MoveList_1_1 {
    });
    label_map.insert("MoveList_1_1", label_106);
    let label_107 = std::rc::Rc::new(nonterminals::Info·0·1··0::Info·0·1··0_0_0::Info·0·1··0_0_0 {});
    label_map.insert("Info·0·1··0_0_0", label_107);
    let label_108 = std::rc::Rc::new(nonterminals::Pokemon·0·4·p::Pokemon·0·4·p {
    });
    label_map.insert("Pokemon·0·4·p", label_108);
    let alt_Pokemon·0·4·p_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Pokemon·0·4·p_0_0".to_string()),
            wagon_ident::Ident::Unknown("Pokemon·0·4·p_0_1".to_string()),
            wagon_ident::Ident::Unknown("Pokemon·0·4·p_0_2".to_string()),
        ],
    );
    rule_map.insert("Pokemon·0·4·p_0", alt_Pokemon·0·4·p_0);
    let alt_Pokemon·0·4·p_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Pokemon·0·4·p_1_0".to_string()),],
    );
    rule_map.insert("Pokemon·0·4·p_1", alt_Pokemon·0·4·p_1);
    let label_109 = std::rc::Rc::new(nonterminals::EVs::EVs_0_2::EVs_0_2 {
    });
    label_map.insert("EVs_0_2", label_109);
    let label_110 = std::rc::Rc::new(nonterminals::EVs::EVs_0_0::EVs_0_0 {
    });
    label_map.insert("EVs_0_0", label_110);
    let label_111 = std::rc::Rc::new(nonterminals::Nature::Nature_1_0::Nature_1_0 {
    });
    label_map.insert("Nature_1_0", label_111);
    let label_112 = std::rc::Rc::new(nonterminals::Nature::Nature_3_0::Nature_3_0 {
    });
    label_map.insert("Nature_3_0", label_112);
    let label_113 = std::rc::Rc::new(nonterminals::Nature::Nature_8_0::Nature_8_0 {
    });
    label_map.insert("Nature_8_0", label_113);
    let label_114 = std::rc::Rc::new(nonterminals::Pokemon·0·4::Pokemon·0·4_0_1::Pokemon·0·4_0_1 {});
    label_map.insert("Pokemon·0·4_0_1", label_114);
    let label_115 = std::rc::Rc::new(nonterminals::Info::Info {});
    label_map.insert("Info", label_115);
    let alt_Info_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info_0_1".to_string()),
            wagon_ident::Ident::Unknown("Info_0_2".to_string()),
            wagon_ident::Ident::Unknown("Info_0_3".to_string()),
            wagon_ident::Ident::Unknown("Info_0_4".to_string()),
        ],
    );
    rule_map.insert("Info_0", alt_Info_0);
    let label_116 = std::rc::Rc::new(nonterminals::Optionals::Optionals_0_1::Optionals_0_1 {});
    label_map.insert("Optionals_0_1", label_116);
    let label_117 = std::rc::Rc::new(nonterminals::Optionals::Optionals_6_1::Optionals_6_1 {});
    label_map.insert("Optionals_6_1", label_117);
    let label_118 = std::rc::Rc::new(nonterminals::Nature::Nature_10_0::Nature_10_0 {
    });
    label_map.insert("Nature_10_0", label_118);
    let label_119 = std::rc::Rc::new(nonterminals::Info·0·1··0::Info·0·1··0 {
    });
    label_map.insert("Info·0·1··0", label_119);
    let alt_Info·0·1··0_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info·0·1··0_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info·0·1··0_0_1".to_string()),
        ],
    );
    rule_map.insert("Info·0·1··0_0", alt_Info·0·1··0_0);
    let label_120 = std::rc::Rc::new(nonterminals::Info·0·1··0::Info·0·1··0_0_1::Info·0·1··0_0_1 {});
    label_map.insert("Info·0·1··0_0_1", label_120);
    let label_121 = std::rc::Rc::new(nonterminals::MoveList::MoveList_0_0::MoveList_0_0 {
    });
    label_map.insert("MoveList_0_0", label_121);
    let label_122 = std::rc::Rc::new(nonterminals::Decimal::Decimal_0_0::Decimal_0_0 {
    });
    label_map.insert("Decimal_0_0", label_122);
    let label_123 = std::rc::Rc::new(nonterminals::Optionals::Optionals_0_0::Optionals_0_0 {});
    label_map.insert("Optionals_0_0", label_123);
    let label_124 = std::rc::Rc::new(nonterminals::EVList::EVList_1_0::EVList_1_0 {
    });
    label_map.insert("EVList_1_0", label_124);
    let label_125 = std::rc::Rc::new(nonterminals::MoveList::MoveList_0_1::MoveList_0_1 {
    });
    label_map.insert("MoveList_0_1", label_125);
    let label_126 = std::rc::Rc::new(nonterminals::AnyString::AnyString_0_0::AnyString_0_0 {});
    label_map.insert("AnyString_0_0", label_126);
    let label_127 = std::rc::Rc::new(nonterminals::Decimal::Decimal_0_1::Decimal_0_1 {
    });
    label_map.insert("Decimal_0_1", label_127);
    let label_128 = std::rc::Rc::new(nonterminals::NumberList::NumberList_0_0::NumberList_0_0 {});
    label_map.insert("NumberList_0_0", label_128);
    let label_129 = std::rc::Rc::new(nonterminals::NumberList::NumberList_1_1::NumberList_1_1 {});
    label_map.insert("NumberList_1_1", label_129);
    let label_130 = std::rc::Rc::new(nonterminals::NumberList::NumberList_1_0::NumberList_1_0 {});
    label_map.insert("NumberList_1_0", label_130);
    let label_131 = std::rc::Rc::new(nonterminals::Number::Number_7_0::Number_7_0 {
    });
    label_map.insert("Number_7_0", label_131);
    let label_132 = std::rc::Rc::new(nonterminals::Stat::Stat {});
    label_map.insert("Stat", label_132);
    let alt_Stat_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Stat_0_0".to_string()),],
    );
    rule_map.insert("Stat_0", alt_Stat_0);
    let alt_Stat_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Stat_1_0".to_string()),],
    );
    rule_map.insert("Stat_1", alt_Stat_1);
    let alt_Stat_2 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Stat_2_0".to_string()),],
    );
    rule_map.insert("Stat_2", alt_Stat_2);
    let alt_Stat_3 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Stat_3_0".to_string()),],
    );
    rule_map.insert("Stat_3", alt_Stat_3);
    let alt_Stat_4 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Stat_4_0".to_string()),],
    );
    rule_map.insert("Stat_4", alt_Stat_4);
    let alt_Stat_5 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Stat_5_0".to_string()),],
    );
    rule_map.insert("Stat_5", alt_Stat_5);
    let label_133 = std::rc::Rc::new(nonterminals::Optionals::Optionals_1_0::Optionals_1_0 {});
    label_map.insert("Optionals_1_0", label_133);
    let label_134 = std::rc::Rc::new(nonterminals::Optionals::Optionals_1_1::Optionals_1_1 {});
    label_map.insert("Optionals_1_1", label_134);
    let label_135 = std::rc::Rc::new(nonterminals::Info::Info_0_4::Info_0_4 {
    });
    label_map.insert("Info_0_4", label_135);
    let label_136 = std::rc::Rc::new(nonterminals::Info·0·3::Info·0·3_0_0::Info·0·3_0_0 {});
    label_map.insert("Info·0·3_0_0", label_136);
    let label_137 = std::rc::Rc::new(nonterminals::Pokemon::Pokemon_0_0::Pokemon_0_0 {
    });
    label_map.insert("Pokemon_0_0", label_137);
    let label_138 = std::rc::Rc::new(nonterminals::YesNo::YesNo {});
    label_map.insert("YesNo", label_138);
    let alt_YesNo_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("YesNo_0_0".to_string()),],
    );
    rule_map.insert("YesNo_0", alt_YesNo_0);
    let alt_YesNo_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("YesNo_1_0".to_string()),],
    );
    rule_map.insert("YesNo_1", alt_YesNo_1);
    let label_139 = std::rc::Rc::new(nonterminals::HappyGuard::HappyGuard {
    });
    label_map.insert("HappyGuard", label_139);
    let alt_HappyGuard_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("HappyGuard_0_0".to_string()),],
    );
    rule_map.insert("HappyGuard_0", alt_HappyGuard_0);
    let label_140 = std::rc::Rc::new(nonterminals::Nature::Nature_2_0::Nature_2_0 {
    });
    label_map.insert("Nature_2_0", label_140);
    let label_141 = std::rc::Rc::new(nonterminals::Nature::Nature_14_0::Nature_14_0 {
    });
    label_map.insert("Nature_14_0", label_141);
    let label_142 = std::rc::Rc::new(nonterminals::Nature::Nature_15_0::Nature_15_0 {
    });
    label_map.insert("Nature_15_0", label_142);
    let label_143 = std::rc::Rc::new(nonterminals::Nature::Nature_22_0::Nature_22_0 {
    });
    label_map.insert("Nature_22_0", label_143);
    let label_144 = std::rc::Rc::new(nonterminals::Info·0·2::Info·0·2_1_0::Info·0·2_1_0 {});
    label_map.insert("Info·0·2_1_0", label_144);
    let label_145 = std::rc::Rc::new(nonterminals::Ability::Ability_0_1::Ability_0_1 {
    });
    label_map.insert("Ability_0_1", label_145);
    let label_146 = std::rc::Rc::new(nonterminals::Info·0·3::Info·0·3_0_1::Info·0·3_0_1 {});
    label_map.insert("Info·0·3_0_1", label_146);
    let label_147 = std::rc::Rc::new(nonterminals::Pokemon·0·4·p::Pokemon·0·4·p_0_2::Pokemon·0·4·p_0_2 {});
    label_map.insert("Pokemon·0·4·p_0_2", label_147);
    let label_148 = std::rc::Rc::new(nonterminals::Item::Item {});
    label_map.insert("Item", label_148);
    let alt_Item_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Item_0_0".to_string()),
            wagon_ident::Ident::Unknown("Item_0_1".to_string()),
        ],
    );
    rule_map.insert("Item_0", alt_Item_0);
    let label_149 = std::rc::Rc::new(nonterminals::Nature::Nature_20_0::Nature_20_0 {
    });
    label_map.insert("Nature_20_0", label_149);
    let label_150 = std::rc::Rc::new(nonterminals::Number::Number_1_0::Number_1_0 {
    });
    label_map.insert("Number_1_0", label_150);
    let label_151 = std::rc::Rc::new(nonterminals::Shiny::Shiny_0_1::Shiny_0_1 {
    });
    label_map.insert("Shiny_0_1", label_151);
    let label_152 = std::rc::Rc::new(nonterminals::PerEVGuard::PerEVGuard {
    });
    label_map.insert("PerEVGuard", label_152);
    let alt_PerEVGuard_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("PerEVGuard_0_0".to_string()),],
    );
    rule_map.insert("PerEVGuard_0", alt_PerEVGuard_0);
    let label_153 = std::rc::Rc::new(nonterminals::Number::Number_9_0::Number_9_0 {
    });
    label_map.insert("Number_9_0", label_153);
    let label_154 = std::rc::Rc::new(nonterminals::Pokemon·0·2::Pokemon·0·2_0_2::Pokemon·0·2_0_2 {});
    label_map.insert("Pokemon·0·2_0_2", label_154);
    let label_155 = std::rc::Rc::new(nonterminals::Nature::Nature_18_0::Nature_18_0 {
    });
    label_map.insert("Nature_18_0", label_155);
    let label_156 = std::rc::Rc::new(nonterminals::Shiny::Shiny {});
    label_map.insert("Shiny", label_156);
    let alt_Shiny_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Shiny_0_0".to_string()),
            wagon_ident::Ident::Unknown("Shiny_0_1".to_string()),
        ],
    );
    rule_map.insert("Shiny_0", alt_Shiny_0);
    let label_157 = std::rc::Rc::new(nonterminals::EVStart::EVStart_0_0::EVStart_0_0 {
    });
    label_map.insert("EVStart_0_0", label_157);
    let label_158 = std::rc::Rc::new(nonterminals::EVs::EVs_0_3::EVs_0_3 {
    });
    label_map.insert("EVs_0_3", label_158);
    let label_159 = std::rc::Rc::new(nonterminals::IVGuard::IVGuard {});
    label_map.insert("IVGuard", label_159);
    let alt_IVGuard_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("IVGuard_0_0".to_string()),],
    );
    rule_map.insert("IVGuard_0", alt_IVGuard_0);
    let label_160 = std::rc::Rc::new(nonterminals::Info·0·3::Info·0·3_1_0::Info·0·3_1_0 {});
    label_map.insert("Info·0·3_1_0", label_160);
    let label_161 = std::rc::Rc::new(nonterminals::Number::Number {});
    label_map.insert("Number", label_161);
    let alt_Number_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_0_0".to_string()),],
    );
    rule_map.insert("Number_0", alt_Number_0);
    let alt_Number_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_1_0".to_string()),],
    );
    rule_map.insert("Number_1", alt_Number_1);
    let alt_Number_2 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_2_0".to_string()),],
    );
    rule_map.insert("Number_2", alt_Number_2);
    let alt_Number_3 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_3_0".to_string()),],
    );
    rule_map.insert("Number_3", alt_Number_3);
    let alt_Number_4 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_4_0".to_string()),],
    );
    rule_map.insert("Number_4", alt_Number_4);
    let alt_Number_5 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_5_0".to_string()),],
    );
    rule_map.insert("Number_5", alt_Number_5);
    let alt_Number_6 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_6_0".to_string()),],
    );
    rule_map.insert("Number_6", alt_Number_6);
    let alt_Number_7 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_7_0".to_string()),],
    );
    rule_map.insert("Number_7", alt_Number_7);
    let alt_Number_8 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_8_0".to_string()),],
    );
    rule_map.insert("Number_8", alt_Number_8);
    let alt_Number_9 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Number_9_0".to_string()),],
    );
    rule_map.insert("Number_9", alt_Number_9);
    let label_162 = std::rc::Rc::new(nonterminals::IVs::IVs_0_3::IVs_0_3 {
    });
    label_map.insert("IVs_0_3", label_162);
    let label_163 = std::rc::Rc::new(nonterminals::Shiny::Shiny_0_0::Shiny_0_0 {
    });
    label_map.insert("Shiny_0_0", label_163);
    let label_164 = std::rc::Rc::new(nonterminals::Info·0·3··0::Info·0·3··0 {
    });
    label_map.insert("Info·0·3··0", label_164);
    let alt_Info·0·3··0_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info·0·3··0_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info·0·3··0_0_1".to_string()),
        ],
    );
    rule_map.insert("Info·0·3··0_0", alt_Info·0·3··0_0);
    let label_165 = std::rc::Rc::new(nonterminals::Pokemon·0·4·p::Pokemon·0·4·p_0_0::Pokemon·0·4·p_0_0 {});
    label_map.insert("Pokemon·0·4·p_0_0", label_165);
    let label_166 = std::rc::Rc::new(nonterminals::EVGuard::EVGuard {});
    label_map.insert("EVGuard", label_166);
    let alt_EVGuard_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("EVGuard_0_0".to_string()),],
    );
    rule_map.insert("EVGuard_0", alt_EVGuard_0);
    let label_167 = std::rc::Rc::new(nonterminals::Level::Level_0_0::Level_0_0 {
    });
    label_map.insert("Level_0_0", label_167);
    let label_168 = std::rc::Rc::new(nonterminals::Nature::Nature_6_0::Nature_6_0 {
    });
    label_map.insert("Nature_6_0", label_168);
    let label_169 = std::rc::Rc::new(nonterminals::Info·0·2··0::Info·0·2··0 {
    });
    label_map.insert("Info·0·2··0", label_169);
    let alt_Info·0·2··0_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info·0·2··0_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info·0·2··0_0_1".to_string()),
        ],
    );
    rule_map.insert("Info·0·2··0_0", alt_Info·0·2··0_0);
    let label_170 = std::rc::Rc::new(nonterminals::EVList::EVList_0_2::EVList_0_2 {
    });
    label_map.insert("EVList_0_2", label_170);
    let label_171 = std::rc::Rc::new(nonterminals::Pokemon·0·4·p::Pokemon·0·4·p_1_0::Pokemon·0·4·p_1_0 {});
    label_map.insert("Pokemon·0·4·p_1_0", label_171);
    let label_172 = std::rc::Rc::new(nonterminals::Number::Number_5_0::Number_5_0 {
    });
    label_map.insert("Number_5_0", label_172);
    let label_173 = std::rc::Rc::new(nonterminals::YesNo::YesNo_0_0::YesNo_0_0 {
    });
    label_map.insert("YesNo_0_0", label_173);
    let label_174 = std::rc::Rc::new(nonterminals::Gender::Gender_0_0::Gender_0_0 {
    });
    label_map.insert("Gender_0_0", label_174);
    let label_175 = std::rc::Rc::new(nonterminals::EVList::EVList_0_0::EVList_0_0 {
    });
    label_map.insert("EVList_0_0", label_175);
    let label_176 = std::rc::Rc::new(nonterminals::Info·0·3··0::Info·0·3··0_0_1::Info·0·3··0_0_1 {});
    label_map.insert("Info·0·3··0_0_1", label_176);
    let label_177 = std::rc::Rc::new(nonterminals::Info·0·3::Info·0·3 {
    });
    label_map.insert("Info·0·3", label_177);
    let alt_Info·0·3_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info·0·3_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info·0·3_0_1".to_string()),
        ],
    );
    rule_map.insert("Info·0·3_0", alt_Info·0·3_0);
    let alt_Info·0·3_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Info·0·3_1_0".to_string()),],
    );
    rule_map.insert("Info·0·3_1", alt_Info·0·3_1);
    let label_178 = std::rc::Rc::new(nonterminals::Name::Name {});
    label_map.insert("Name", label_178);
    let alt_Name_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Name_0_0".to_string()),
            wagon_ident::Ident::Unknown("Name_0_1".to_string()),
        ],
    );
    rule_map.insert("Name_0", alt_Name_0);
    let label_179 = std::rc::Rc::new(nonterminals::Item::Item_0_1::Item_0_1 {
    });
    label_map.insert("Item_0_1", label_179);
    let label_180 = std::rc::Rc::new(nonterminals::Happiness::Happiness_0_0::Happiness_0_0 {});
    label_map.insert("Happiness_0_0", label_180);
    let label_181 = std::rc::Rc::new(nonterminals::Nature::Nature_7_0::Nature_7_0 {
    });
    label_map.insert("Nature_7_0", label_181);
    let label_182 = std::rc::Rc::new(nonterminals::IVStart::IVStart_0_0::IVStart_0_0 {
    });
    label_map.insert("IVStart_0_0", label_182);
    let label_183 = std::rc::Rc::new(nonterminals::NatureDef::NatureDef_0_1::NatureDef_0_1 {});
    label_map.insert("NatureDef_0_1", label_183);
    let label_184 = std::rc::Rc::new(nonterminals::Nature::Nature_16_0::Nature_16_0 {
    });
    label_map.insert("Nature_16_0", label_184);
    let label_185 = std::rc::Rc::new(nonterminals::LevelGuard::LevelGuard_0_0::LevelGuard_0_0 {});
    label_map.insert("LevelGuard_0_0", label_185);
    let label_186 = std::rc::Rc::new(nonterminals::EVStart::EVStart {});
    label_map.insert("EVStart", label_186);
    let alt_EVStart_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("EVStart_0_0".to_string()),
            wagon_ident::Ident::Unknown("EVStart_0_1".to_string()),
        ],
    );
    rule_map.insert("EVStart_0", alt_EVStart_0);
    let label_187 = std::rc::Rc::new(nonterminals::IVs::IVs_0_1::IVs_0_1 {
    });
    label_map.insert("IVs_0_1", label_187);
    let label_188 = std::rc::Rc::new(nonterminals::Info·0·1::Info·0·1 {
    });
    label_map.insert("Info·0·1", label_188);
    let alt_Info·0·1_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("Info·0·1_0_0".to_string()),
            wagon_ident::Ident::Unknown("Info·0·1_0_1".to_string()),
        ],
    );
    rule_map.insert("Info·0·1_0", alt_Info·0·1_0);
    let alt_Info·0·1_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("Info·0·1_1_0".to_string()),],
    );
    rule_map.insert("Info·0·1_1", alt_Info·0·1_1);
    let label_189 = std::rc::Rc::new(nonterminals::Pokemon::Pokemon_0_2::Pokemon_0_2 {
    });
    label_map.insert("Pokemon_0_2", label_189);
    let label_190 = std::rc::Rc::new(nonterminals::Optionals::Optionals_4_1::Optionals_4_1 {});
    label_map.insert("Optionals_4_1", label_190);
    let label_191 = std::rc::Rc::new(nonterminals::Stat::Stat_1_0::Stat_1_0 {
    });
    label_map.insert("Stat_1_0", label_191);
    let label_192 = std::rc::Rc::new(nonterminals::LevelGuard::LevelGuard {
    });
    label_map.insert("LevelGuard", label_192);
    let alt_LevelGuard_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("LevelGuard_0_0".to_string()),],
    );
    rule_map.insert("LevelGuard_0", alt_LevelGuard_0);
    let label_193 = std::rc::Rc::new(nonterminals::Info::Info_0_3::Info_0_3 {
    });
    label_map.insert("Info_0_3", label_193);
    let label_194 = std::rc::Rc::new(nonterminals::EVList::EVList {});
    label_map.insert("EVList", label_194);
    let alt_EVList_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("EVList_0_0".to_string()),
            wagon_ident::Ident::Unknown("EVList_0_1".to_string()),
            wagon_ident::Ident::Unknown("EVList_0_2".to_string()),
        ],
    );
    rule_map.insert("EVList_0", alt_EVList_0);
    let alt_EVList_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("EVList_1_0".to_string()),
            wagon_ident::Ident::Unknown("EVList_1_1".to_string()),
        ],
    );
    rule_map.insert("EVList_1", alt_EVList_1);
    let label_195 = std::rc::Rc::new(nonterminals::Number::Number_8_0::Number_8_0 {
    });
    label_map.insert("Number_8_0", label_195);
    static ALIGNED_0: &regex_automata::util::wire::AlignAs<[u8], u32> = &regex_automata::util::wire::AlignAs {
        _align: [],
        #[cfg(target_endian = "big")]
        bytes: *include_bytes!("regexes/7ca58d3faada540a_big.dfa"),
        #[cfg(target_endian = "little")]
        bytes: *include_bytes!("regexes/7ca58d3faada540a_little.dfa"),
    };
    let (dfa, _) = regex_automata::dfa::dense::DFA::from_bytes(&ALIGNED_0.bytes)
        .expect("Unable to serialize regex DFA");
    let automata = wagon_gll::RegexTerminal::new("[a-zA-Z]+([ -]*[a-zA-Z]+)*", dfa);
    let pointer = std::rc::Rc::new(automata);
    label_map.insert("[a-zA-Z]+([ -]*[a-zA-Z]+)*", pointer.clone());
    regex_map.insert("[a-zA-Z]+([ -]*[a-zA-Z]+)*", pointer);
    let mut state = wagon_gll::GLLState::init(contents, label_map, rule_map, regex_map)
        .unwrap();
    state.main();
    match state.print_sppf_dot(crop, math_mode) {
        Ok(t) => println!("{t}"),
        Err(e) => println!("Error: {e}"),
    }
    if print_gss {
        match state.print_gss_dot(math_mode) {
            Ok(t) => println!("{t}"),
            Err(e) => println!("Error: {e}"),
        }
    }
    let offset = content_string.len() - contents.len();
    if state.final_accepts() {
        let mut real_errors = Vec::new();
        for error in state.errors {
            match error {
                wagon_gll::GLLError::ParseError(_) => {}
                other => real_errors.push(other),
            }
        }
        if !real_errors.is_empty() {
            wagon_utils::handle_error(
                    real_errors,
                    input_file_str,
                    &content_string,
                    offset,
                )
                .unwrap()
        }
    } else {
        wagon_utils::handle_error(state.errors, input_file_str, &content_string, offset)
            .unwrap()
    }
}

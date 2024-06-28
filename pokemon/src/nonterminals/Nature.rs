#[path = "Nature/Nature_0_0.rs"]
pub(crate) mod Nature_0_0;
#[path = "Nature/Nature_1_0.rs"]
pub(crate) mod Nature_1_0;
#[path = "Nature/Nature_2_0.rs"]
pub(crate) mod Nature_2_0;
#[path = "Nature/Nature_3_0.rs"]
pub(crate) mod Nature_3_0;
#[path = "Nature/Nature_4_0.rs"]
pub(crate) mod Nature_4_0;
#[path = "Nature/Nature_5_0.rs"]
pub(crate) mod Nature_5_0;
#[path = "Nature/Nature_6_0.rs"]
pub(crate) mod Nature_6_0;
#[path = "Nature/Nature_7_0.rs"]
pub(crate) mod Nature_7_0;
#[path = "Nature/Nature_8_0.rs"]
pub(crate) mod Nature_8_0;
#[path = "Nature/Nature_9_0.rs"]
pub(crate) mod Nature_9_0;
#[path = "Nature/Nature_10_0.rs"]
pub(crate) mod Nature_10_0;
#[path = "Nature/Nature_11_0.rs"]
pub(crate) mod Nature_11_0;
#[path = "Nature/Nature_12_0.rs"]
pub(crate) mod Nature_12_0;
#[path = "Nature/Nature_13_0.rs"]
pub(crate) mod Nature_13_0;
#[path = "Nature/Nature_14_0.rs"]
pub(crate) mod Nature_14_0;
#[path = "Nature/Nature_15_0.rs"]
pub(crate) mod Nature_15_0;
#[path = "Nature/Nature_16_0.rs"]
pub(crate) mod Nature_16_0;
#[path = "Nature/Nature_17_0.rs"]
pub(crate) mod Nature_17_0;
#[path = "Nature/Nature_18_0.rs"]
pub(crate) mod Nature_18_0;
#[path = "Nature/Nature_19_0.rs"]
pub(crate) mod Nature_19_0;
#[path = "Nature/Nature_20_0.rs"]
pub(crate) mod Nature_20_0;
#[path = "Nature/Nature_21_0.rs"]
pub(crate) mod Nature_21_0;
#[path = "Nature/Nature_22_0.rs"]
pub(crate) mod Nature_22_0;
#[path = "Nature/Nature_23_0.rs"]
pub(crate) mod Nature_23_0;
#[path = "Nature/Nature_24_0.rs"]
pub(crate) mod Nature_24_0;
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Nature;
impl<'a> wagon_gll::Label<'a> for Nature {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(
            vec![
                (vec![state.get_label_by_uuid("Nature_0_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_1_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_2_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_3_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_4_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_5_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_6_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_7_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_8_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_9_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_10_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_11_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_12_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_13_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_14_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_15_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_16_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_17_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_18_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_19_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_20_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_21_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_22_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_23_0") ?], None), (vec![state
                .get_label_by_uuid("Nature_24_0") ?], None),
            ],
        )
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Nature"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let mut candidates = Vec::with_capacity(25usize);
        let alt_count = 25usize;
        let mut zero_weights = 0;
        let fst = state.get_label_by_uuid("Nature_0_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_0")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_0_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_1_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_1")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_1_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_2_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_2")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_2_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_3_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_3")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_3_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_4_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_4")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_4_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_5_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_5")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_5_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_6_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_6")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_6_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_7_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_7")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_7_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_8_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_8")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_8_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_9_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_9")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_9_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_10_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_10")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_10_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_11_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_11")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_11_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_12_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_12")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_12_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_13_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_13")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_13_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_14_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_14")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_14_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_15_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_15")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_15_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_16_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_16")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_16_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_17_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_17")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_17_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_18_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_18")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_18_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_19_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_19")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_19_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_20_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_20")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_20_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_21_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_21")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_21_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_22_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_22")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_22_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_23_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_23")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_23_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Nature_24_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Nature")?;
            let rules = state.get_rule("Nature_24")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Nature_24_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        if !candidates.is_empty() {
            let to_add = wagon_utils::FallibleItertools::fallible_max_set_by(
                candidates.into_iter(),
                |(x, _), (y, _)| {
                    x
                        .partial_cmp(y)
                        .map_or_else(
                            || Err(
                                wagon_gll::GLLImplementationError::ValueError(
                                    wagon_gll::value::ValueError::ValueError(
                                        wagon_value::ValueError::ComparisonError(
                                            x.to_owned(),
                                            y.to_owned(),
                                        ),
                                    ),
                                ),
                            ),
                            Ok,
                        )
                },
            )?;
            for (_, slot) in to_add {
                state
                    .add(
                        slot,
                        state.gss_pointer,
                        state.input_pointer,
                        state.sppf_root,
                        state.gss_pointer,
                    );
            }
        } else if zero_weights == alt_count {
            return Err(
                wagon_gll::GLLError::ParseError(wagon_gll::GLLParseError::ZeroWeights {
                    pointer: state.input_pointer + 1,
                    rule: self.to_string().to_owned(),
                    context: state
                        .get_current_gss_node()?
                        .get_slot()
                        .to_string(state, false),
                }),
            )
        } else {
            return Err(
                wagon_gll::GLLError::ParseError(wagon_gll::GLLParseError::NoCandidates {
                    pointer: state.input_pointer + 1,
                    rule: self.to_string().to_owned(),
                    context: state
                        .get_current_gss_node()?
                        .get_slot()
                        .to_string(state, false),
                }),
            )
        }
        Ok(())
    }
    fn to_string(&self) -> &str {
        "Nature"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Nature",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec![])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        Some(
            Err(
                wagon_gll::GLLImplementationError::Fatal(
                    "Weight should never be evaluated for non-zero GLL blocks",
                ),
            ),
        )
    }
}

#[path = "Stat/Stat_0_0.rs"]
pub(crate) mod Stat_0_0;
#[path = "Stat/Stat_1_0.rs"]
pub(crate) mod Stat_1_0;
#[path = "Stat/Stat_2_0.rs"]
pub(crate) mod Stat_2_0;
#[path = "Stat/Stat_3_0.rs"]
pub(crate) mod Stat_3_0;
#[path = "Stat/Stat_4_0.rs"]
pub(crate) mod Stat_4_0;
#[path = "Stat/Stat_5_0.rs"]
pub(crate) mod Stat_5_0;
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Stat;
impl<'a> wagon_gll::Label<'a> for Stat {
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
                (vec![state.get_label_by_uuid("Stat_0_0") ?], None), (vec![state
                .get_label_by_uuid("Stat_1_0") ?], None), (vec![state
                .get_label_by_uuid("Stat_2_0") ?], None), (vec![state
                .get_label_by_uuid("Stat_3_0") ?], None), (vec![state
                .get_label_by_uuid("Stat_4_0") ?], None), (vec![state
                .get_label_by_uuid("Stat_5_0") ?], None),
            ],
        )
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Stat"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let mut candidates = Vec::with_capacity(6usize);
        let alt_count = 6usize;
        let mut zero_weights = 0;
        let fst = state.get_label_by_uuid("Stat_0_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Stat")?;
            let rules = state.get_rule("Stat_0")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Stat_0_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Stat_1_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Stat")?;
            let rules = state.get_rule("Stat_1")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Stat_1_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Stat_2_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Stat")?;
            let rules = state.get_rule("Stat_2")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Stat_2_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Stat_3_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Stat")?;
            let rules = state.get_rule("Stat_3")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Stat_3_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Stat_4_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Stat")?;
            let rules = state.get_rule("Stat_4")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Stat_4_0");
            let weight = slot.weight(state)?;
            if wagon_value::Valueable::is_truthy(&weight)? {
                candidates.push((weight, std::rc::Rc::new(slot)));
            } else {
                zero_weights = zero_weights + 1;
            }
        }
        let fst = state.get_label_by_uuid("Stat_5_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Stat")?;
            let rules = state.get_rule("Stat_5")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Stat_5_0");
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
        "Stat"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Stat",]
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

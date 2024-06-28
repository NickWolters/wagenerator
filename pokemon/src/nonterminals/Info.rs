#[path = "Info/Info_0_0.rs"]
pub(crate) mod Info_0_0;
#[path = "Info/Info_0_1.rs"]
pub(crate) mod Info_0_1;
#[path = "Info/Info_0_2.rs"]
pub(crate) mod Info_0_2;
#[path = "Info/Info_0_3.rs"]
pub(crate) mod Info_0_3;
#[path = "Info/Info_0_4.rs"]
pub(crate) mod Info_0_4;
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info;
impl<'a> wagon_gll::Label<'a> for Info {
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
                (vec![state.get_label_by_uuid("Info_0_0") ?, state
                .get_label_by_uuid("Info_0_1") ?, state.get_label_by_uuid("Info_0_2") ?,
                state.get_label_by_uuid("Info_0_3") ?, state
                .get_label_by_uuid("Info_0_4") ?], None),
            ],
        )
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Info"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let mut candidates = Vec::with_capacity(1usize);
        let alt_count = 1usize;
        let mut zero_weights = 0;
        let fst = state.get_label_by_uuid("Info_0_0")?;
        if state.test_next(&fst)? {
            let root = state.get_label_by_uuid("Info")?;
            let rules = state.get_rule("Info_0")?;
            let slot = wagon_gll::GrammarSlot::new(root, rules, 0, 0, "Info_0_0");
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
        "Info"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Info",]
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

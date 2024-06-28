#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct EVList_0_1;
impl<'a> wagon_gll::Label<'a> for EVList_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"/")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "EVList_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_ev_count = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(2usize)?
        }
            .to_owned();
        let s_total = if let Some(v) = state.get_ret_val(1usize)? {
            v
        } else {
            state.restore_attribute(3usize)?
        }
            .to_owned();
        let i = state.input_pointer;
        let bytes = b"/";
        if state.has_next(bytes) {
            state.next(bytes)?;
            let node = state.get_node_t(bytes, i, state.input_pointer);
            let slot = wagon_gll::GrammarSlot::new(
                state.get_label_by_uuid("EVList")?,
                state.get_rule("EVList_0")?,
                1usize,
                1usize,
                "EVList_0",
            );
            state
                .sppf_pointer = state
                .get_node_p(
                    std::rc::Rc::new(slot),
                    state.sppf_pointer,
                    node,
                    state.gss_pointer,
                    false,
                )?;
        } else {
            return Ok(())
        }
        let label = state.get_label(&wagon_ident::Ident::Unknown("EVList".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("EVList")?,
                            state.get_rule("EVList_0")?,
                            2usize,
                            0,
                            "EVList_0",
                        ),
                    ),
                    vec![s_ev_count.clone(), s_total.clone(), s_ev_count, s_total,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "'/' EVList<&ev_count, &total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'/'", "EVList<&ev_count, &total>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["&ev_count", "&total",], vec!["&ev_count", "&total",])
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

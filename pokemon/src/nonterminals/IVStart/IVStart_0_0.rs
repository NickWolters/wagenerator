#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct IVStart_0_0;
impl<'a> wagon_gll::Label<'a> for IVStart_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"IVs")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "IVStart_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_iv_count = state.get_attribute(0usize)?.to_owned();
        let i = state.input_pointer;
        let bytes = b"IVs";
        state.next(bytes)?;
        let new_node = state.get_node_t(bytes, i, state.input_pointer);
        state.sppf_pointer = new_node;
        let i = state.input_pointer;
        let bytes = b":";
        if state.has_next(bytes) {
            state.next(bytes)?;
            let node = state.get_node_t(bytes, i, state.input_pointer);
            let slot = wagon_gll::GrammarSlot::new(
                state.get_label_by_uuid("IVStart")?,
                state.get_rule("IVStart_0")?,
                0usize,
                2usize,
                "IVStart_0",
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
        let label = state.get_label(&wagon_ident::Ident::Unknown("IVList".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("IVStart")?,
                            state.get_rule("IVStart_0")?,
                            1usize,
                            0,
                            "IVStart_0",
                        ),
                    ),
                    vec![s_iv_count.clone(), s_iv_count,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "'IVs' ':' IVList<&iv_count>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'IVs'", "':'", "IVList<&iv_count>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&iv_count",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

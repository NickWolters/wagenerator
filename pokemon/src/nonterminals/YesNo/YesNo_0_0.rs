#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct YesNo_0_0;
impl<'a> wagon_gll::Label<'a> for YesNo_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"Yes")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "YesNo_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_yes = state.get_attribute(0usize)?.to_owned();
        let i = state.input_pointer;
        let bytes = b"Yes";
        state.next(bytes)?;
        let node = state.get_node_t(bytes, i, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("YesNo")?,
            state.get_rule("YesNo_0")?,
            1usize,
            0usize,
            "YesNo_0",
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
        let s_yes: wagon_gll::value::Value = (wagon_value::Value::from(true)).into();
        Ok(state.pop(&vec![Some(s_yes.clone()),], vec![s_yes,])?)
    }
    fn to_string(&self) -> &str {
        "'Yes'"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'Yes'",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&yes",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

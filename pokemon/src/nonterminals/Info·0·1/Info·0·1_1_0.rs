#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info·0·1_1_0;
impl<'a> wagon_gll::Label<'a> for Info·0·1_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(& [])),])
    }
    fn is_eps(&self) -> bool {
        true
    }
    fn uuid(&self) -> &str {
        "Info·0·1_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_nickname = state.get_attribute(0usize)?.to_owned();
        let cr = state.get_node_t(&[], state.input_pointer, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("Info·0·1")?,
            state.get_rule("Info·0·1_1")?,
            1,
            0,
            "Info·0·1_1",
        );
        state
            .sppf_pointer = state
            .get_node_p(
                std::rc::Rc::new(slot),
                state.sppf_pointer,
                cr,
                state.gss_pointer,
                false,
            )?;
        Ok(state.pop(&vec![Some(s_nickname.clone()),], vec![s_nickname,])?)
    }
    fn to_string(&self) -> &str {
        "ε"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["ε",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&nickname",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct AnyString_0_0;
impl<'a> wagon_gll::Label<'a> for AnyString_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("[a-zA-Z]+([ -]*[a-zA-Z]+)*") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "AnyString_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let i = state.input_pointer;
        let pattern = "[a-zA-Z]+([ -]*[a-zA-Z]+)*";
        let bytes = state
            .next_regex(pattern)?
            .ok_or_else(|| wagon_gll::GLLImplementationError::Fatal(
                "Failed to get match with regex, even though we already checked.",
            ))?;
        let node = state.get_node_t(bytes, i, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("AnyString")?,
            state.get_rule("AnyString_0")?,
            1usize,
            0usize,
            "AnyString_0",
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
        Ok(state.pop(&vec![], vec![])?)
    }
    fn to_string(&self) -> &str {
        "/[a-zA-Z]+([ -]*[a-zA-Z]+)*/"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["/[a-zA-Z]+([ -]*[a-zA-Z]+)*/",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec![])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

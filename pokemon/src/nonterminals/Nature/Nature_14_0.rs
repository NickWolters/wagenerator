#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Nature_14_0;
impl<'a> wagon_gll::Label<'a> for Nature_14_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"Quiet")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Nature_14_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let i = state.input_pointer;
        let bytes = b"Quiet";
        state.next(bytes)?;
        let node = state.get_node_t(bytes, i, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("Nature")?,
            state.get_rule("Nature_14")?,
            1usize,
            0usize,
            "Nature_14",
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
        "'Quiet'"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'Quiet'",]
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

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Number_3_0;
impl<'a> wagon_gll::Label<'a> for Number_3_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"3")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Number_3_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_value = state.get_attribute(0usize)?.to_owned();
        let i = state.input_pointer;
        let bytes = b"3";
        state.next(bytes)?;
        let node = state.get_node_t(bytes, i, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("Number")?,
            state.get_rule("Number_3")?,
            1usize,
            0usize,
            "Number_3",
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
        let s_value: wagon_gll::value::Value = (wagon_value::Value::from(3i32)).into();
        Ok(state.pop(&vec![Some(s_value.clone()),], vec![s_value,])?)
    }
    fn to_string(&self) -> &str {
        "'3'"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'3'",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&value",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

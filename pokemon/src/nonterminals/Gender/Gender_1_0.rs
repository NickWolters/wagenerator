#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Gender_1_0;
impl<'a> wagon_gll::Label<'a> for Gender_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"F")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Gender_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_gender = state.get_attribute(0usize)?.to_owned();
        let i = state.input_pointer;
        let bytes = b"F";
        state.next(bytes)?;
        let node = state.get_node_t(bytes, i, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("Gender")?,
            state.get_rule("Gender_1")?,
            1usize,
            0usize,
            "Gender_1",
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
        let s_gender: wagon_gll::value::Value = (wagon_value::Value::from("F")).into();
        Ok(state.pop(&vec![Some(s_gender.clone()),], vec![s_gender,])?)
    }
    fn to_string(&self) -> &str {
        "'F'"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'F'",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&gender",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

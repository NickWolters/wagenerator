#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info·0·1··0_0_1;
impl<'a> wagon_gll::Label<'a> for Info·0·1··0_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Info·0·1··0_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_nickname = state.restore_attribute(0usize)?.clone();
        let s_nickname: wagon_gll::value::Value = (wagon_value::Value::from(true))
            .into();
        let i = state.input_pointer;
        let bytes = b")";
        if state.has_next(bytes) {
            state.next(bytes)?;
            let node = state.get_node_t(bytes, i, state.input_pointer);
            let slot = wagon_gll::GrammarSlot::new(
                state.get_label_by_uuid("Info·0·1··0")?,
                state.get_rule("Info·0·1··0_0")?,
                2usize,
                0usize,
                "Info·0·1··0_0",
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
        Ok(state.pop(&vec![Some(s_nickname.clone()),], vec![s_nickname,])?)
    }
    fn to_string(&self) -> &str {
        "')'"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["')'",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&nickname",])
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

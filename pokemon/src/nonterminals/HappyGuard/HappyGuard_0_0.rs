#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct HappyGuard_0_0;
impl<'a> wagon_gll::Label<'a> for HappyGuard_0_0 {
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
        "HappyGuard_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let i_total = state.get_attribute(0usize)?.to_owned();
        let cr = state.get_node_t(&[], state.input_pointer, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("HappyGuard")?,
            state.get_rule("HappyGuard_0")?,
            1,
            0,
            "HappyGuard_0",
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
        Ok(state.pop(&vec![None,], vec![i_total,])?)
    }
    fn to_string(&self) -> &str {
        "ε"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["ε",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["*total",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        fn actual_weight<'a>(
            state: &wagon_gll::GLLState<'a>,
        ) -> wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>> {
            let i_total = state.get_attribute(0usize)?.to_owned();
            Ok(
                wagon_gll::value::Value::from(
                    wagon_value::Valueable::is_truthy(
                        &wagon_value::Value::from(
                            (i_total.clone() >= wagon_value::Value::from(0i32)),
                        ),
                    )?
                        && wagon_value::Valueable::is_truthy(
                            &wagon_value::Value::from(
                                (i_total.clone() <= wagon_value::Value::from(255i32)),
                            ),
                        )?,
                ),
            )
        }
        Some(actual_weight(state))
    }
}

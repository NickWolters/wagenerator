#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct E_1_0;
impl<'a> wagon_gll::Label<'a> for E_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("D") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "E_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let i_did_a = state.get_attribute(0usize)?.to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("D".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("E")?,
                        state.get_rule("E_1")?,
                        1usize,
                        0,
                        "E_1",
                    ),
                ),
                vec![i_did_a,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "D"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["D",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["*did_a",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        fn actual_weight<'a>(
            state: &wagon_gll::GLLState<'a>,
        ) -> wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>> {
            let i_did_a = state.get_attribute(0usize)?.to_owned();
            Ok(
                wagon_gll::value::Value::from(
                    std::ops::Sub::sub(
                        std::ops::Mul::mul(
                            (std::ops::Not::not(i_did_a.clone())?),
                            wagon_value::Value::from(3i32),
                        )?,
                        wagon_value::Value::from(4i32),
                    )?,
                ),
            )
        }
        Some(actual_weight(state))
    }
}

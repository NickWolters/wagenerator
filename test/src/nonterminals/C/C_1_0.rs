#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct C_1_0;
impl<'a> wagon_gll::Label<'a> for C_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("G") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "C_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let label = state.get_label(&wagon_ident::Ident::Unknown("G".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("C")?,
                        state.get_rule("C_1")?,
                        1usize,
                        0,
                        "C_1",
                    ),
                ),
                vec![],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "G"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["G",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec![])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        fn actual_weight<'a>(
            state: &wagon_gll::GLLState<'a>,
        ) -> wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>> {
            Ok(wagon_gll::value::Value::from(wagon_value::Value::try_from(0.7f32)?))
        }
        Some(actual_weight(state))
    }
}

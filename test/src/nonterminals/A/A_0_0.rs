#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct A_0_0;
impl<'a> wagon_gll::Label<'a> for A_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("E") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "A_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let l_did_a: wagon_gll::value::Value = (wagon_value::Value::from(true)).into();
        let label = state.get_label(&wagon_ident::Ident::Unknown("E".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("A")?,
                        state.get_rule("A_0")?,
                        1usize,
                        0,
                        "A_0",
                    ),
                ),
                vec![l_did_a.clone(), l_did_a,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "E<$did_a>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["E<$did_a>",]
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

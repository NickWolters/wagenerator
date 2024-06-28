#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct NumberList_1_0;
impl<'a> wagon_gll::Label<'a> for NumberList_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Number") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "NumberList_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_total = state.get_attribute(0usize)?.to_owned();
        let l_value: wagon_gll::value::Value = (wagon_value::Value::from(0i32)).into();
        let label = state.get_label(&wagon_ident::Ident::Unknown("Number".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("NumberList")?,
                        state.get_rule("NumberList_1")?,
                        1usize,
                        0,
                        "NumberList_1",
                    ),
                ),
                vec![l_value.clone(), s_total, l_value,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "Number<$value>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Number<$value>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&total",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

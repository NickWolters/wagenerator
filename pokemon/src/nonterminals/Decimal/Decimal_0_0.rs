#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Decimal_0_0;
impl<'a> wagon_gll::Label<'a> for Decimal_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("NumberList") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Decimal_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_total = state.get_attribute(0usize)?.to_owned();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("NumberList".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("Decimal")?,
                        state.get_rule("Decimal_0")?,
                        1usize,
                        0,
                        "Decimal_0",
                    ),
                ),
                vec![s_total.clone(), s_total,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "NumberList<&total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["NumberList<&total>",]
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

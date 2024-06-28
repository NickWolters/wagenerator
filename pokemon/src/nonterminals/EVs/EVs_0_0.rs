#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct EVs_0_0;
impl<'a> wagon_gll::Label<'a> for EVs_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Decimal") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "EVs_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_total = state.get_attribute(1usize)?.to_owned();
        let s_ev_count = state.get_attribute(0usize)?.to_owned();
        let l_total: wagon_gll::value::Value = (wagon_value::Value::from(0i32)).into();
        let label = state.get_label(&wagon_ident::Ident::Unknown("Decimal".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("EVs")?,
                        state.get_rule("EVs_0")?,
                        1usize,
                        0,
                        "EVs_0",
                    ),
                ),
                vec![l_total.clone(), s_ev_count, s_total, l_total,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "Decimal<$total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Decimal<$total>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&ev_count", "&total",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        fn actual_weight<'a>(
            state: &wagon_gll::GLLState<'a>,
        ) -> wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>> {
            let s_ev_count = state.get_attribute(0usize)?.to_owned();
            Ok(
                wagon_gll::value::Value::from(
                    wagon_value::Value::from(
                        (s_ev_count.clone() < wagon_value::Value::from(6i32)),
                    ),
                ),
            )
        }
        Some(actual_weight(state))
    }
}

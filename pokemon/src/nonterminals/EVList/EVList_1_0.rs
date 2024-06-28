#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct EVList_1_0;
impl<'a> wagon_gll::Label<'a> for EVList_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("EVs") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "EVList_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_ev_count = state.get_attribute(0usize)?.to_owned();
        let s_total = state.get_attribute(1usize)?.to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("EVs".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("EVList")?,
                        state.get_rule("EVList_1")?,
                        1usize,
                        0,
                        "EVList_1",
                    ),
                ),
                vec![s_ev_count.clone(), s_total.clone(), s_ev_count, s_total,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "EVs<&ev_count, &total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["EVs<&ev_count, &total>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&ev_count", "&total",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

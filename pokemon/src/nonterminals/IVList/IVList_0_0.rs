#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct IVList_0_0;
impl<'a> wagon_gll::Label<'a> for IVList_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("IVs") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "IVList_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_iv_count = state.get_attribute(0usize)?.to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("IVs".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("IVList")?,
                        state.get_rule("IVList_0")?,
                        1usize,
                        0,
                        "IVList_0",
                    ),
                ),
                vec![s_iv_count.clone(), s_iv_count,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "IVs<&iv_count>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["IVs<&iv_count>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&iv_count",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

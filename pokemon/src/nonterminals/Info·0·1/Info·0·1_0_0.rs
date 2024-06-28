#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info·0·1_0_0;
impl<'a> wagon_gll::Label<'a> for Info·0·1_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Info·0·1··0") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Info·0·1_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_nickname = state.get_attribute(0usize)?.to_owned();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Info·0·1··0".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("Info·0·1")?,
                        state.get_rule("Info·0·1_0")?,
                        1usize,
                        0,
                        "Info·0·1_0",
                    ),
                ),
                vec![s_nickname.clone(), s_nickname,],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "Info·0·1··0<&nickname>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Info·0·1··0<&nickname>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&nickname",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

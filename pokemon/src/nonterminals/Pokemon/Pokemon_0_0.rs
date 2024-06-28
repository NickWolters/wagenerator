#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon_0_0;
impl<'a> wagon_gll::Label<'a> for Pokemon_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Info") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Pokemon_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_nickname = state.get_attribute(1usize)?.to_owned();
        let s_shiny = state.get_attribute(0usize)?.to_owned();
        let s_gender = state.get_attribute(2usize)?.to_owned();
        let s_item = state.get_attribute(3usize)?.to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("Info".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("Pokemon")?,
                        state.get_rule("Pokemon_0")?,
                        1usize,
                        0,
                        "Pokemon_0",
                    ),
                ),
                vec![
                    s_nickname.clone(), s_gender.clone(), s_item.clone(), s_shiny,
                    s_nickname, s_gender, s_item,
                ],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "Info<&nickname, &gender, &item>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Info<&nickname, &gender, &item>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&shiny", "&nickname", "&gender", "&item",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

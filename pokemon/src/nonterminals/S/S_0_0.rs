#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct S_0_0;
impl<'a> wagon_gll::Label<'a> for S_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Pokemon") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "S_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let l_shiny: wagon_gll::value::Value = (wagon_value::Value::from(false)).into();
        let l_nickname: wagon_gll::value::Value = (wagon_value::Value::from(false))
            .into();
        let l_gender: wagon_gll::value::Value = (wagon_value::Value::from("U")).into();
        let l_item: wagon_gll::value::Value = (wagon_value::Value::from(false)).into();
        let label = state.get_label(&wagon_ident::Ident::Unknown("Pokemon".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("S")?,
                        state.get_rule("S_0")?,
                        1usize,
                        0,
                        "S_0",
                    ),
                ),
                vec![
                    l_shiny.clone(), l_nickname.clone(), l_gender.clone(), l_item
                    .clone(), l_shiny, l_nickname, l_gender, l_item,
                ],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "Pokemon<$shiny, $nickname, $gender, $item>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Pokemon<$shiny, $nickname, $gender, $item>",]
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

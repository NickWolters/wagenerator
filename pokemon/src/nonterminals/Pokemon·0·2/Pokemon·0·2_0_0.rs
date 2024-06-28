#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon·0·2_0_0;
impl<'a> wagon_gll::Label<'a> for Pokemon·0·2_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Optionals") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Pokemon·0·2_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_d_ability = state.get_attribute(0usize)?.to_owned();
        let s_d_iv = state.get_attribute(6usize)?.to_owned();
        let s_d_level = state.get_attribute(2usize)?.to_owned();
        let s_d_nat = state.get_attribute(4usize)?.to_owned();
        let s_d_happy = state.get_attribute(3usize)?.to_owned();
        let s_d_shiny = state.get_attribute(1usize)?.to_owned();
        let s_d_ev = state.get_attribute(5usize)?.to_owned();
        let s_shiny = state.get_attribute(7usize)?.to_owned();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Optionals".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("Pokemon·0·2")?,
                        state.get_rule("Pokemon·0·2_0")?,
                        1usize,
                        0,
                        "Pokemon·0·2_0",
                    ),
                ),
                vec![
                    s_d_ability.clone(), s_d_shiny.clone(), s_d_level.clone(), s_d_happy
                    .clone(), s_d_nat.clone(), s_d_ev.clone(), s_d_iv.clone(), s_shiny
                    .clone(), s_d_ability, s_d_shiny, s_d_level, s_d_happy, s_d_nat,
                    s_d_ev, s_d_iv, s_shiny,
                ],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "Optionals<&d_ability, &d_shiny, &d_level, &d_happy, &d_nat, &d_ev, &d_iv, &shiny>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![
            "Optionals<&d_ability, &d_shiny, &d_level, &d_happy, &d_nat, &d_ev, &d_iv, &shiny>",
        ]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (
            vec![],
            vec![
                "&d_ability", "&d_shiny", "&d_level", "&d_happy", "&d_nat", "&d_ev",
                "&d_iv", "&shiny",
            ],
        )
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

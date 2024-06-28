#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon·0·2_1_0;
impl<'a> wagon_gll::Label<'a> for Pokemon·0·2_1_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(& [])),])
    }
    fn is_eps(&self) -> bool {
        true
    }
    fn uuid(&self) -> &str {
        "Pokemon·0·2_1_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_shiny = state.get_attribute(7usize)?.to_owned();
        let s_d_ev = state.get_attribute(5usize)?.to_owned();
        let s_d_level = state.get_attribute(2usize)?.to_owned();
        let s_d_nat = state.get_attribute(4usize)?.to_owned();
        let s_d_iv = state.get_attribute(6usize)?.to_owned();
        let s_d_happy = state.get_attribute(3usize)?.to_owned();
        let s_d_ability = state.get_attribute(0usize)?.to_owned();
        let s_d_shiny = state.get_attribute(1usize)?.to_owned();
        let cr = state.get_node_t(&[], state.input_pointer, state.input_pointer);
        let slot = wagon_gll::GrammarSlot::new(
            state.get_label_by_uuid("Pokemon·0·2")?,
            state.get_rule("Pokemon·0·2_1")?,
            1,
            0,
            "Pokemon·0·2_1",
        );
        state
            .sppf_pointer = state
            .get_node_p(
                std::rc::Rc::new(slot),
                state.sppf_pointer,
                cr,
                state.gss_pointer,
                false,
            )?;
        Ok(
            state
                .pop(
                    &vec![
                        Some(s_d_ability.clone()), Some(s_d_shiny.clone()),
                        Some(s_d_level.clone()), Some(s_d_happy.clone()), Some(s_d_nat
                        .clone()), Some(s_d_ev.clone()), Some(s_d_iv.clone()),
                        Some(s_shiny.clone()),
                    ],
                    vec![
                        s_d_ability, s_d_shiny, s_d_level, s_d_happy, s_d_nat, s_d_ev,
                        s_d_iv, s_shiny,
                    ],
                )?,
        )
    }
    fn to_string(&self) -> &str {
        "ε"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["ε",]
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

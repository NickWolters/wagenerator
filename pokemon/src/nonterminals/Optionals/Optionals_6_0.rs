#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Optionals_6_0;
impl<'a> wagon_gll::Label<'a> for Optionals_6_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("EVStart") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Optionals_6_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_d_level = state.get_attribute(2usize)?.to_owned();
        let s_shiny = state.get_attribute(7usize)?.to_owned();
        let s_d_ability = state.get_attribute(0usize)?.to_owned();
        let s_d_nat = state.get_attribute(4usize)?.to_owned();
        let s_d_happy = state.get_attribute(3usize)?.to_owned();
        let s_d_ev = state.get_attribute(5usize)?.to_owned();
        let s_d_shiny = state.get_attribute(1usize)?.to_owned();
        let s_d_iv = state.get_attribute(6usize)?.to_owned();
        let l_ev_count: wagon_gll::value::Value = (wagon_value::Value::from(0i32))
            .into();
        let l_ev_total: wagon_gll::value::Value = (wagon_value::Value::from(0i32))
            .into();
        let label = state.get_label(&wagon_ident::Ident::Unknown("EVStart".to_string()));
        state
            .gss_pointer = state
            .create(
                &std::rc::Rc::new(
                    wagon_gll::GrammarSlot::new(
                        state.get_label_by_uuid("Optionals")?,
                        state.get_rule("Optionals_6")?,
                        1usize,
                        0,
                        "Optionals_6",
                    ),
                ),
                vec![
                    l_ev_count.clone(), l_ev_total.clone(), s_d_ability, s_d_shiny,
                    s_d_level, s_d_happy, s_d_nat, s_d_ev, s_d_iv, s_shiny, l_ev_count,
                    l_ev_total,
                ],
            )?;
        label.code(state)
    }
    fn to_string(&self) -> &str {
        "EVStart<$ev_count, $ev_total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["EVStart<$ev_count, $ev_total>",]
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
        fn actual_weight<'a>(
            state: &wagon_gll::GLLState<'a>,
        ) -> wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>> {
            let s_d_ev = state.get_attribute(5usize)?.to_owned();
            Ok(wagon_gll::value::Value::from(std::ops::Not::not(s_d_ev.clone())?))
        }
        Some(actual_weight(state))
    }
}

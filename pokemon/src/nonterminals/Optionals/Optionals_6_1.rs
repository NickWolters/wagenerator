#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Optionals_6_1;
impl<'a> wagon_gll::Label<'a> for Optionals_6_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("EVGuard") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Optionals_6_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_d_ability = state.restore_attribute(2usize)?.clone();
        let s_d_level = state.restore_attribute(4usize)?.clone();
        let s_d_nat = state.restore_attribute(6usize)?.clone();
        let s_d_happy = state.restore_attribute(5usize)?.clone();
        let s_d_iv = state.restore_attribute(8usize)?.clone();
        let s_shiny = state.restore_attribute(9usize)?.clone();
        let l_ev_total = if let Some(v) = state.get_ret_val(1usize)? {
            v
        } else {
            state.restore_attribute(11usize)?
        }
            .to_owned();
        let s_d_shiny = state.restore_attribute(3usize)?.clone();
        let s_d_ev = state.restore_attribute(7usize)?.clone();
        let l_ev_count = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(10usize)?
        }
            .to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("EVGuard".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Optionals")?,
                            state.get_rule("Optionals_6")?,
                            2usize,
                            0,
                            "Optionals_6",
                        ),
                    ),
                    vec![
                        l_ev_total.clone(), s_d_ability, s_d_shiny, s_d_level, s_d_happy,
                        s_d_nat, s_d_ev, s_d_iv, s_shiny, l_ev_count, l_ev_total,
                    ],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "EVGuard<$ev_total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["EVGuard<$ev_total>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (
            vec!["$ev_count", "$ev_total",],
            vec![
                "&d_ability", "&d_shiny", "&d_level", "&d_happy", "&d_nat", "&d_ev",
                "&d_iv", "&shiny", "$ev_count", "$ev_total",
            ],
        )
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        Some(
            Err(
                wagon_gll::GLLImplementationError::Fatal(
                    "Weight should never be evaluated for non-zero GLL blocks",
                ),
            ),
        )
    }
}

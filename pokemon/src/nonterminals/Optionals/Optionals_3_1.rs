#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Optionals_3_1;
impl<'a> wagon_gll::Label<'a> for Optionals_3_1 {
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
        "Optionals_3_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_d_shiny = state.restore_attribute(1usize)?.clone();
        let s_d_iv = state.restore_attribute(6usize)?.clone();
        let s_shiny = state.restore_attribute(7usize)?.clone();
        let s_d_happy = state.restore_attribute(3usize)?.clone();
        let s_d_ability = state.restore_attribute(0usize)?.clone();
        let s_d_level = state.restore_attribute(2usize)?.clone();
        let s_d_ev = state.restore_attribute(5usize)?.clone();
        let s_d_nat = state.restore_attribute(4usize)?.clone();
        let s_d_happy: wagon_gll::value::Value = (wagon_value::Value::from(true)).into();
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
        ""
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![]
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
        Some(
            Err(
                wagon_gll::GLLImplementationError::Fatal(
                    "Weight should never be evaluated for non-zero GLL blocks",
                ),
            ),
        )
    }
}

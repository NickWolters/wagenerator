#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct EVs_0_3;
impl<'a> wagon_gll::Label<'a> for EVs_0_3 {
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
        "EVs_0_3"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let l_total = state.restore_attribute(2usize)?.clone();
        let s_total = state.restore_attribute(1usize)?.clone();
        let s_ev_count = state.restore_attribute(0usize)?.clone();
        let s_ev_count: wagon_gll::value::Value = (std::ops::Add::add(
            s_ev_count.clone(),
            wagon_value::Value::from(1i32),
        )?)
            .into();
        Ok(
            state
                .pop(
                    &vec![Some(s_ev_count.clone()), Some(s_total.clone()), None,],
                    vec![s_ev_count, s_total, l_total,],
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
        (vec![], vec!["&ev_count", "&total", "$total",])
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

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct EVs_0_2;
impl<'a> wagon_gll::Label<'a> for EVs_0_2 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Stat") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "EVs_0_2"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_total = state.restore_attribute(2usize)?.clone();
        let s_ev_count = state.restore_attribute(1usize)?.clone();
        let l_total = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(3usize)?
        }
            .to_owned();
        let s_total: wagon_gll::value::Value = (std::ops::Add::add(
            s_total.clone(),
            l_total.clone(),
        )?)
            .into();
        let label = state.get_label(&wagon_ident::Ident::Unknown("Stat".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("EVs")?,
                            state.get_rule("EVs_0")?,
                            3usize,
                            0,
                            "EVs_0",
                        ),
                    ),
                    vec![s_ev_count, s_total, l_total,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Stat"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Stat",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["$total",], vec!["&ev_count", "&total", "$total",])
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

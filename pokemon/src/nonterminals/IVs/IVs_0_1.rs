#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct IVs_0_1;
impl<'a> wagon_gll::Label<'a> for IVs_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("IVGuard") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "IVs_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_iv_count = state.restore_attribute(1usize)?.clone();
        let l_total = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(2usize)?
        }
            .to_owned();
        let label = state.get_label(&wagon_ident::Ident::Unknown("IVGuard".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("IVs")?,
                            state.get_rule("IVs_0")?,
                            2usize,
                            0,
                            "IVs_0",
                        ),
                    ),
                    vec![l_total.clone(), s_iv_count, l_total,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "IVGuard<$total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["IVGuard<$total>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["$total",], vec!["&iv_count", "$total",])
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

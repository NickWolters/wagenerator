#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct NumberList_0_1;
impl<'a> wagon_gll::Label<'a> for NumberList_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("NumberList") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "NumberList_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let l_value = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(2usize)?
        }
            .to_owned();
        let s_total = state.restore_attribute(1usize)?.clone();
        let s_total: wagon_gll::value::Value = (std::ops::Add::add(
            std::ops::Mul::mul(s_total.clone(), wagon_value::Value::from(10i32))?,
            l_value.clone(),
        )?)
            .into();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("NumberList".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("NumberList")?,
                            state.get_rule("NumberList_0")?,
                            2usize,
                            0,
                            "NumberList_0",
                        ),
                    ),
                    vec![s_total.clone(), s_total, l_value,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "NumberList<&total>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["NumberList<&total>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["$value",], vec!["&total", "$value",])
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

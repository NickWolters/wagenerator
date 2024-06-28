#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon·0·4_0_1;
impl<'a> wagon_gll::Label<'a> for Pokemon·0·4_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Pokemon·0·4·p") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Pokemon·0·4_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_move_count = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(1usize)?
        }
            .to_owned();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Pokemon·0·4·p".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Pokemon·0·4")?,
                            state.get_rule("Pokemon·0·4_0")?,
                            2usize,
                            0,
                            "Pokemon·0·4_0",
                        ),
                    ),
                    vec![s_move_count.clone(), s_move_count,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Pokemon·0·4·p<&move_count>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Pokemon·0·4·p<&move_count>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["&move_count",], vec!["&move_count",])
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

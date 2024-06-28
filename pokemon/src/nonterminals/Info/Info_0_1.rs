#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info_0_1;
impl<'a> wagon_gll::Label<'a> for Info_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Info·0·1") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Info_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_item = state.restore_attribute(2usize)?.clone();
        let s_gender = state.restore_attribute(1usize)?.clone();
        let s_nickname = state.restore_attribute(0usize)?.clone();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Info·0·1".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Info")?,
                            state.get_rule("Info_0")?,
                            2usize,
                            0,
                            "Info_0",
                        ),
                    ),
                    vec![s_nickname.clone(), s_nickname, s_gender, s_item,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Info·0·1<&nickname>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Info·0·1<&nickname>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&nickname", "&gender", "&item",])
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

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info_0_3;
impl<'a> wagon_gll::Label<'a> for Info_0_3 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Info·0·3") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Info_0_3"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_gender = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(2usize)?
        }
            .to_owned();
        let s_item = state.restore_attribute(3usize)?.clone();
        let s_nickname = state.restore_attribute(1usize)?.clone();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Info·0·3".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Info")?,
                            state.get_rule("Info_0")?,
                            4usize,
                            0,
                            "Info_0",
                        ),
                    ),
                    vec![s_item.clone(), s_nickname, s_gender, s_item,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Info·0·3<&item>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Info·0·3<&item>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec!["&gender",], vec!["&nickname", "&gender", "&item",])
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

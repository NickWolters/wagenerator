#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Info·0·3··0_0_0;
impl<'a> wagon_gll::Label<'a> for Info·0·3··0_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"@")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Info·0·3··0_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_item = state.get_attribute(0usize)?.to_owned();
        let i = state.input_pointer;
        let bytes = b"@";
        state.next(bytes)?;
        let new_node = state.get_node_t(bytes, i, state.input_pointer);
        state.sppf_pointer = new_node;
        let label = state.get_label(&wagon_ident::Ident::Unknown("Item".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Info·0·3··0")?,
                            state.get_rule("Info·0·3··0_0")?,
                            1usize,
                            0,
                            "Info·0·3··0_0",
                        ),
                    ),
                    vec![s_item,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "'@' Item"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'@'", "Item",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&item",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Shiny_0_0;
impl<'a> wagon_gll::Label<'a> for Shiny_0_0 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(b"Shiny")),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Shiny_0_0"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_shiny = state.get_attribute(0usize)?.to_owned();
        let i = state.input_pointer;
        let bytes = b"Shiny";
        state.next(bytes)?;
        let new_node = state.get_node_t(bytes, i, state.input_pointer);
        state.sppf_pointer = new_node;
        let i = state.input_pointer;
        let bytes = b":";
        if state.has_next(bytes) {
            state.next(bytes)?;
            let node = state.get_node_t(bytes, i, state.input_pointer);
            let slot = wagon_gll::GrammarSlot::new(
                state.get_label_by_uuid("Shiny")?,
                state.get_rule("Shiny_0")?,
                0usize,
                2usize,
                "Shiny_0",
            );
            state
                .sppf_pointer = state
                .get_node_p(
                    std::rc::Rc::new(slot),
                    state.sppf_pointer,
                    node,
                    state.gss_pointer,
                    false,
                )?;
        } else {
            return Ok(())
        }
        let label = state.get_label(&wagon_ident::Ident::Unknown("YesNo".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Shiny")?,
                            state.get_rule("Shiny_0")?,
                            1usize,
                            0,
                            "Shiny_0",
                        ),
                    ),
                    vec![s_shiny.clone(), s_shiny,],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "'Shiny' ':' YesNo<&shiny>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["'Shiny'", "':'", "YesNo<&shiny>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (vec![], vec!["&shiny",])
    }
    #[allow(unused_variables)]
    fn _weight(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        None
    }
}

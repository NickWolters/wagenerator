#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon·0·2_0_1;
impl<'a> wagon_gll::Label<'a> for Pokemon·0·2_0_1 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Pokemon·0·2") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Pokemon·0·2_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_d_ev = if let Some(v) = state.get_ret_val(5usize)? {
            v
        } else {
            state.restore_attribute(13usize)?
        }
            .to_owned();
        let s_d_level = if let Some(v) = state.get_ret_val(2usize)? {
            v
        } else {
            state.restore_attribute(10usize)?
        }
            .to_owned();
        let s_d_shiny = if let Some(v) = state.get_ret_val(1usize)? {
            v
        } else {
            state.restore_attribute(9usize)?
        }
            .to_owned();
        let s_shiny = if let Some(v) = state.get_ret_val(7usize)? {
            v
        } else {
            state.restore_attribute(15usize)?
        }
            .to_owned();
        let s_d_nat = if let Some(v) = state.get_ret_val(4usize)? {
            v
        } else {
            state.restore_attribute(12usize)?
        }
            .to_owned();
        let s_d_ability = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(8usize)?
        }
            .to_owned();
        let s_d_iv = if let Some(v) = state.get_ret_val(6usize)? {
            v
        } else {
            state.restore_attribute(14usize)?
        }
            .to_owned();
        let s_d_happy = if let Some(v) = state.get_ret_val(3usize)? {
            v
        } else {
            state.restore_attribute(11usize)?
        }
            .to_owned();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Pokemon·0·2".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Pokemon·0·2")?,
                            state.get_rule("Pokemon·0·2_0")?,
                            2usize,
                            0,
                            "Pokemon·0·2_0",
                        ),
                    ),
                    vec![
                        s_d_ability.clone(), s_d_shiny.clone(), s_d_level.clone(),
                        s_d_happy.clone(), s_d_nat.clone(), s_d_ev.clone(), s_d_iv
                        .clone(), s_shiny.clone(), s_d_ability, s_d_shiny, s_d_level,
                        s_d_happy, s_d_nat, s_d_ev, s_d_iv, s_shiny,
                    ],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Pokemon·0·2<&d_ability, &d_shiny, &d_level, &d_happy, &d_nat, &d_ev, &d_iv, &shiny>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![
            "Pokemon·0·2<&d_ability, &d_shiny, &d_level, &d_happy, &d_nat, &d_ev, &d_iv, &shiny>",
        ]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (
            vec![
                "&d_ability", "&d_shiny", "&d_level", "&d_happy", "&d_nat", "&d_ev",
                "&d_iv", "&shiny",
            ],
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

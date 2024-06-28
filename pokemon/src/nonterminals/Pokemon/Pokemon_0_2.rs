#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon_0_2;
impl<'a> wagon_gll::Label<'a> for Pokemon_0_2 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("Pokemon·0·4") ?], None),])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        "Pokemon_0_2"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_item = state.restore_attribute(11usize)?.clone();
        let l_d_ev = if let Some(v) = state.get_ret_val(5usize)? {
            v
        } else {
            state.restore_attribute(17usize)?
        }
            .to_owned();
        let l_d_iv = if let Some(v) = state.get_ret_val(6usize)? {
            v
        } else {
            state.restore_attribute(18usize)?
        }
            .to_owned();
        let l_d_level = if let Some(v) = state.get_ret_val(2usize)? {
            v
        } else {
            state.restore_attribute(14usize)?
        }
            .to_owned();
        let s_nickname = state.restore_attribute(9usize)?.clone();
        let l_d_happy = if let Some(v) = state.get_ret_val(3usize)? {
            v
        } else {
            state.restore_attribute(15usize)?
        }
            .to_owned();
        let l_d_shiny = if let Some(v) = state.get_ret_val(1usize)? {
            v
        } else {
            state.restore_attribute(13usize)?
        }
            .to_owned();
        let l_d_nat = if let Some(v) = state.get_ret_val(4usize)? {
            v
        } else {
            state.restore_attribute(16usize)?
        }
            .to_owned();
        let l_d_ability = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(12usize)?
        }
            .to_owned();
        let s_shiny = if let Some(v) = state.get_ret_val(7usize)? {
            v
        } else {
            state.restore_attribute(8usize)?
        }
            .to_owned();
        let s_gender = state.restore_attribute(10usize)?.clone();
        let l_move_count: wagon_gll::value::Value = (wagon_value::Value::from(0i32))
            .into();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Pokemon·0·4".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Pokemon")?,
                            state.get_rule("Pokemon_0")?,
                            3usize,
                            0,
                            "Pokemon_0",
                        ),
                    ),
                    vec![
                        l_move_count.clone(), s_shiny, s_nickname, s_gender, s_item,
                        l_d_ability, l_d_shiny, l_d_level, l_d_happy, l_d_nat, l_d_ev,
                        l_d_iv, l_move_count,
                    ],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Pokemon·0·4<$move_count>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec!["Pokemon·0·4<$move_count>",]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (
            vec![
                "&shiny", "$d_ability", "$d_shiny", "$d_level", "$d_happy", "$d_nat",
                "$d_ev", "$d_iv",
            ],
            vec![
                "&shiny", "&nickname", "&gender", "&item", "$d_ability", "$d_shiny",
                "$d_level", "$d_happy", "$d_nat", "$d_ev", "$d_iv",
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

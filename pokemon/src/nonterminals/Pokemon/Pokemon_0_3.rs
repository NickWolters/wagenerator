#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon_0_3;
impl<'a> wagon_gll::Label<'a> for Pokemon_0_3 {
    #[allow(unused_variables)]
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![], Some(& [])),])
    }
    fn is_eps(&self) -> bool {
        true
    }
    fn uuid(&self) -> &str {
        "Pokemon_0_3"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_shiny = state.restore_attribute(1usize)?.clone();
        let l_d_ability = state.restore_attribute(5usize)?.clone();
        let l_d_happy = state.restore_attribute(8usize)?.clone();
        let l_move_count = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(12usize)?
        }
            .to_owned();
        let s_nickname = state.restore_attribute(2usize)?.clone();
        let l_d_ev = state.restore_attribute(10usize)?.clone();
        let l_d_nat = state.restore_attribute(9usize)?.clone();
        let l_d_shiny = state.restore_attribute(6usize)?.clone();
        let s_gender = state.restore_attribute(3usize)?.clone();
        let l_d_level = state.restore_attribute(7usize)?.clone();
        let l_d_iv = state.restore_attribute(11usize)?.clone();
        let s_item = state.restore_attribute(4usize)?.clone();
        Ok(
            state
                .pop(
                    &vec![
                        Some(s_shiny.clone()), Some(s_nickname.clone()), Some(s_gender
                        .clone()), Some(s_item.clone()), None, None, None, None, None,
                        None, None, None,
                    ],
                    vec![
                        s_shiny, s_nickname, s_gender, s_item, l_d_ability, l_d_shiny,
                        l_d_level, l_d_happy, l_d_nat, l_d_ev, l_d_iv, l_move_count,
                    ],
                )?,
        )
    }
    fn to_string(&self) -> &str {
        ""
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (
            vec!["$move_count",],
            vec![
                "&shiny", "&nickname", "&gender", "&item", "$d_ability", "$d_shiny",
                "$d_level", "$d_happy", "$d_nat", "$d_ev", "$d_iv", "$move_count",
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

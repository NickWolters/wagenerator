#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct S_0_1;
impl<'a> wagon_gll::Label<'a> for S_0_1 {
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
        "S_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let l_item = if let Some(v) = state.get_ret_val(3usize)? {
            v
        } else {
            state.restore_attribute(7usize)?
        }
            .to_owned();
        let l_shiny = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(4usize)?
        }
            .to_owned();
        let l_gender = if let Some(v) = state.get_ret_val(2usize)? {
            v
        } else {
            state.restore_attribute(6usize)?
        }
            .to_owned();
        let l_nickname = if let Some(v) = state.get_ret_val(1usize)? {
            v
        } else {
            state.restore_attribute(5usize)?
        }
            .to_owned();
        Ok(
            state
                .pop(
                    &vec![None, None, None, None,],
                    vec![l_shiny, l_nickname, l_gender, l_item,],
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
            vec!["$shiny", "$nickname", "$gender", "$item",],
            vec!["$shiny", "$nickname", "$gender", "$item",],
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

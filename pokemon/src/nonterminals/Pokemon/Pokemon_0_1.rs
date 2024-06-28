#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) struct Pokemon_0_1;
impl<'a> wagon_gll::Label<'a> for Pokemon_0_1 {
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
        "Pokemon_0_1"
    }
    #[allow(unused_variables)]
    fn code(&self, state: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        let s_nickname = if let Some(v) = state.get_ret_val(0usize)? {
            v
        } else {
            state.restore_attribute(4usize)?
        }
            .to_owned();
        let s_gender = if let Some(v) = state.get_ret_val(1usize)? {
            v
        } else {
            state.restore_attribute(5usize)?
        }
            .to_owned();
        let s_item = if let Some(v) = state.get_ret_val(2usize)? {
            v
        } else {
            state.restore_attribute(6usize)?
        }
            .to_owned();
        let s_shiny = state.restore_attribute(3usize)?.clone();
        let l_d_ability: wagon_gll::value::Value = (wagon_value::Value::from(false))
            .into();
        let l_d_shiny: wagon_gll::value::Value = (wagon_value::Value::from(false))
            .into();
        let l_d_level: wagon_gll::value::Value = (wagon_value::Value::from(false))
            .into();
        let l_d_happy: wagon_gll::value::Value = (wagon_value::Value::from(false))
            .into();
        let l_d_nat: wagon_gll::value::Value = (wagon_value::Value::from(false)).into();
        let l_d_ev: wagon_gll::value::Value = (wagon_value::Value::from(false)).into();
        let l_d_iv: wagon_gll::value::Value = (wagon_value::Value::from(false)).into();
        let label = state
            .get_label(&wagon_ident::Ident::Unknown("Pokemon·0·2".to_string()));
        if state.test_next(&label)? {
            state
                .gss_pointer = state
                .create(
                    &std::rc::Rc::new(
                        wagon_gll::GrammarSlot::new(
                            state.get_label_by_uuid("Pokemon")?,
                            state.get_rule("Pokemon_0")?,
                            2usize,
                            0,
                            "Pokemon_0",
                        ),
                    ),
                    vec![
                        l_d_ability.clone(), l_d_shiny.clone(), l_d_level.clone(),
                        l_d_happy.clone(), l_d_nat.clone(), l_d_ev.clone(), l_d_iv
                        .clone(), s_shiny.clone(), s_shiny, s_nickname, s_gender, s_item,
                        l_d_ability, l_d_shiny, l_d_level, l_d_happy, l_d_nat, l_d_ev,
                        l_d_iv,
                    ],
                )?;
            label.code(state)
        } else {
            Ok(())
        }
    }
    fn to_string(&self) -> &str {
        "Pokemon·0·2<$d_ability, $d_shiny, $d_level, $d_happy, $d_nat, $d_ev, $d_iv, &shiny>"
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![
            "Pokemon·0·2<$d_ability, $d_shiny, $d_level, $d_happy, $d_nat, $d_ev, $d_iv, &shiny>",
        ]
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (
            vec!["&nickname", "&gender", "&item",],
            vec!["&shiny", "&nickname", "&gender", "&item",],
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

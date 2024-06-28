mod nonterminals;
#[derive(Debug)]
struct _S;
impl<'a> wagon_gll::Label<'a> for _S {
    fn first_set(
        &self,
        state: &wagon_gll::GLLState<'a>,
    ) -> wagon_gll::ImplementationResult<
        'a,
        Vec<(Vec<wagon_gll::GLLBlockLabel<'a>>, Option<wagon_gll::Terminal<'a>>)>,
    > {
        Ok(vec![(vec![state.get_label_by_uuid("S") ?], None)])
    }
    fn is_eps(&self) -> bool {
        false
    }
    fn uuid(&self) -> &str {
        wagon_gll::ROOT_UUID
    }
    fn to_string(&self) -> &str {
        wagon_gll::ROOT_UUID
    }
    fn str_parts(&self) -> Vec<&str> {
        vec![wagon_gll::ROOT_UUID]
    }
    fn code(&self, _: &mut wagon_gll::GLLState<'a>) -> wagon_gll::GLLResult<'a, ()> {
        unreachable!("This should never be called");
    }
    fn attr_rep_map(&self) -> (Vec<&str>, Vec<&str>) {
        (Vec::new(), Vec::new())
    }
    fn _weight(
        &self,
        _state: &wagon_gll::GLLState<'a>,
    ) -> Option<wagon_gll::ImplementationResult<'a, wagon_gll::value::Value<'a>>> {
        unreachable!("This should never be called");
    }
}
#[allow(non_snake_case, unused_mut)]
fn main() {
    let args = clap::command!()
        .arg(
            clap::arg!(< filename > "Input file to parse")
                .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
        .arg(clap::arg!(- - "no-crop" "Don't crop resulting sppf").num_args(0))
        .arg(
            clap::arg!(
                - - "math-mode" "Print SPPF dot labels in Latex math-mode representation"
            )
                .num_args(0),
        )
        .arg(
            clap::arg!(- - "print-gss" "Also print the final GSS (works with math-mode)")
                .num_args(0),
        )
        .get_matches();
    let input_file = args
        .get_one::<std::path::PathBuf>("filename")
        .expect("Input file required");
    let input_file_str = Box::leak(input_file.to_str().unwrap().into());
    let crop = args.get_one::<bool>("no-crop").unwrap_or(&false) == &false;
    let math_mode = *args.get_one::<bool>("math-mode").unwrap_or(&false);
    let print_gss = *args.get_one::<bool>("print-gss").unwrap_or(&false);
    let content_string = std::fs::read_to_string(input_file)
        .expect("Couldn't read file");
    let contents: &'static [u8] = Box::leak(content_string.trim().as_bytes().into());
    let mut label_map: wagon_gll::LabelMap = std::collections::HashMap::with_capacity(
        34usize,
    );
    let mut rule_map: wagon_gll::RuleMap = std::collections::HashMap::with_capacity(
        8usize,
    );
    let mut regex_map: wagon_gll::RegexMap = std::collections::HashMap::with_capacity(
        0usize,
    );
    let label_0 = std::rc::Rc::new(nonterminals::A::A_0_2::A_0_2 {});
    label_map.insert("A_0_2", label_0);
    let label_1 = std::rc::Rc::new(nonterminals::S::S_1_0::S_1_0 {});
    label_map.insert("S_1_0", label_1);
    let label_2 = std::rc::Rc::new(nonterminals::C::C {});
    label_map.insert("C", label_2);
    let alt_C_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("C_0_0".to_string()),
            wagon_ident::Ident::Unknown("C_0_1".to_string()),
        ],
    );
    rule_map.insert("C_0", alt_C_0);
    let alt_C_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("C_1_0".to_string()),
            wagon_ident::Ident::Unknown("C_1_1".to_string()),
        ],
    );
    rule_map.insert("C_1", alt_C_1);
    let label_3 = std::rc::Rc::new(nonterminals::B::B_0_1::B_0_1 {});
    label_map.insert("B_0_1", label_3);
    let label_4 = std::rc::Rc::new(nonterminals::C::C_0_1::C_0_1 {});
    label_map.insert("C_0_1", label_4);
    let label_5 = std::rc::Rc::new(nonterminals::F::F_0_0::F_0_0 {});
    label_map.insert("F_0_0", label_5);
    let label_6 = std::rc::Rc::new(nonterminals::G::G {});
    label_map.insert("G", label_6);
    let alt_G_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("G_0_0".to_string()),],
    );
    rule_map.insert("G_0", alt_G_0);
    let label_7 = std::rc::Rc::new(nonterminals::D::D_1_1::D_1_1 {});
    label_map.insert("D_1_1", label_7);
    let label_8 = std::rc::Rc::new(nonterminals::A::A_0_0::A_0_0 {});
    label_map.insert("A_0_0", label_8);
    let label_9 = std::rc::Rc::new(nonterminals::A::A_0_1::A_0_1 {});
    label_map.insert("A_0_1", label_9);
    let label_10 = std::rc::Rc::new(nonterminals::E::E_1_1::E_1_1 {});
    label_map.insert("E_1_1", label_10);
    let label_11 = std::rc::Rc::new(nonterminals::D::D_1_0::D_1_0 {});
    label_map.insert("D_1_0", label_11);
    let label_12 = std::rc::Rc::new(nonterminals::C::C_1_0::C_1_0 {});
    label_map.insert("C_1_0", label_12);
    let label_13 = std::rc::Rc::new(nonterminals::B::B {});
    label_map.insert("B", label_13);
    let alt_B_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("B_0_0".to_string()),
            wagon_ident::Ident::Unknown("B_0_1".to_string()),
            wagon_ident::Ident::Unknown("B_0_2".to_string()),
        ],
    );
    rule_map.insert("B_0", alt_B_0);
    let alt_B_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("B_1_0".to_string()),],
    );
    rule_map.insert("B_1", alt_B_1);
    let label_14 = std::rc::Rc::new(nonterminals::E::E {});
    label_map.insert("E", label_14);
    let alt_E_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("E_0_0".to_string()),
            wagon_ident::Ident::Unknown("E_0_1".to_string()),
        ],
    );
    rule_map.insert("E_0", alt_E_0);
    let alt_E_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("E_1_0".to_string()),
            wagon_ident::Ident::Unknown("E_1_1".to_string()),
        ],
    );
    rule_map.insert("E_1", alt_E_1);
    let label_15 = std::rc::Rc::new(nonterminals::D::D {});
    label_map.insert("D", label_15);
    let alt_D_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("D_0_0".to_string()),
            wagon_ident::Ident::Unknown("D_0_1".to_string()),
        ],
    );
    rule_map.insert("D_0", alt_D_0);
    let alt_D_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("D_1_0".to_string()),
            wagon_ident::Ident::Unknown("D_1_1".to_string()),
        ],
    );
    rule_map.insert("D_1", alt_D_1);
    let label_16 = std::rc::Rc::new(nonterminals::E::E_0_0::E_0_0 {});
    label_map.insert("E_0_0", label_16);
    let label_17 = std::rc::Rc::new(nonterminals::S::S_0_0::S_0_0 {});
    label_map.insert("S_0_0", label_17);
    let label_18 = std::rc::Rc::new(nonterminals::G::G_0_0::G_0_0 {});
    label_map.insert("G_0_0", label_18);
    let label_19 = std::rc::Rc::new(nonterminals::C::C_1_1::C_1_1 {});
    label_map.insert("C_1_1", label_19);
    let label_20 = std::rc::Rc::new(nonterminals::E::E_1_0::E_1_0 {});
    label_map.insert("E_1_0", label_20);
    let label_21 = std::rc::Rc::new(nonterminals::S::S_1_1::S_1_1 {});
    label_map.insert("S_1_1", label_21);
    let label_22 = std::rc::Rc::new(nonterminals::B::B_1_0::B_1_0 {});
    label_map.insert("B_1_0", label_22);
    let label_23 = std::rc::Rc::new(nonterminals::B::B_0_0::B_0_0 {});
    label_map.insert("B_0_0", label_23);
    let label_24 = std::rc::Rc::new(nonterminals::B::B_0_2::B_0_2 {});
    label_map.insert("B_0_2", label_24);
    let label_25 = std::rc::Rc::new(nonterminals::A::A_1_0::A_1_0 {});
    label_map.insert("A_1_0", label_25);
    let label_26 = std::rc::Rc::new(nonterminals::E::E_0_1::E_0_1 {});
    label_map.insert("E_0_1", label_26);
    let label_27 = std::rc::Rc::new(nonterminals::F::F {});
    label_map.insert("F", label_27);
    let alt_F_0 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("F_0_0".to_string()),],
    );
    rule_map.insert("F_0", alt_F_0);
    let label_28 = std::rc::Rc::new(nonterminals::A::A {});
    label_map.insert("A", label_28);
    let alt_A_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("A_0_0".to_string()),
            wagon_ident::Ident::Unknown("A_0_1".to_string()),
            wagon_ident::Ident::Unknown("A_0_2".to_string()),
        ],
    );
    rule_map.insert("A_0", alt_A_0);
    let alt_A_1 = std::rc::Rc::new(
        vec![wagon_ident::Ident::Unknown("A_1_0".to_string()),],
    );
    rule_map.insert("A_1", alt_A_1);
    let label_29 = std::rc::Rc::new(nonterminals::D::D_0_0::D_0_0 {});
    label_map.insert("D_0_0", label_29);
    let label_30 = std::rc::Rc::new(nonterminals::D::D_0_1::D_0_1 {});
    label_map.insert("D_0_1", label_30);
    let label_31 = std::rc::Rc::new(nonterminals::S::S_0_1::S_0_1 {});
    label_map.insert("S_0_1", label_31);
    let label_32 = std::rc::Rc::new(nonterminals::C::C_0_0::C_0_0 {});
    label_map.insert("C_0_0", label_32);
    let label_33 = std::rc::Rc::new(nonterminals::S::S {});
    label_map.insert("S", label_33);
    let alt_S_0 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("S_0_0".to_string()),
            wagon_ident::Ident::Unknown("S_0_1".to_string()),
        ],
    );
    rule_map.insert("S_0", alt_S_0);
    let alt_S_1 = std::rc::Rc::new(
        vec![
            wagon_ident::Ident::Unknown("S_1_0".to_string()),
            wagon_ident::Ident::Unknown("S_1_1".to_string()),
        ],
    );
    rule_map.insert("S_1", alt_S_1);
    label_map.insert(wagon_gll::ROOT_UUID, std::rc::Rc::new(_S {}));
    rule_map
        .insert(
            wagon_gll::ROOT_UUID,
            std::rc::Rc::new(vec![wagon_ident::Ident::Unknown("S".to_string())]),
        );
    let mut state = wagon_gll::GLLState::init(contents, label_map, rule_map, regex_map)
        .unwrap();
    state.main();
    match state.print_sppf_dot(crop, math_mode) {
        Ok(t) => println!("{t}"),
        Err(e) => println!("Error: {e}"),
    }
    if print_gss {
        match state.print_gss_dot(math_mode) {
            Ok(t) => println!("{t}"),
            Err(e) => println!("Error: {e}"),
        }
    }
    let offset = content_string.len() - contents.len();
    if state.final_accepts() {
        let mut real_errors = Vec::new();
        for error in state.errors {
            match error {
                wagon_gll::GLLError::ParseError(_) => {}
                other => real_errors.push(other),
            }
        }
        if !real_errors.is_empty() {
            wagon_utils::handle_error(
                    real_errors,
                    input_file_str,
                    &content_string,
                    offset,
                )
                .unwrap()
        }
    } else {
        wagon_utils::handle_error(state.errors, input_file_str, &content_string, offset)
            .unwrap()
    }
}

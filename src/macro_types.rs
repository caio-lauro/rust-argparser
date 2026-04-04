macro_rules! define_arg_types {
    ($($variant:ident => $type:ty),* $(,)?) => {
        /// Enum used to store the type of the argument.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum ArgumentType {
            $($variant),*
        }

        /// Enum used to store both the type and the value of the argument.
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum ParsedValue {
            $($variant($type)),*
        }

        impl ParsedValue {
            pub fn matches(&self, argtype: &ArgumentType) -> bool {
                matches!(
                    (&self, argtype),
                    $((ParsedValue::$variant(_), ArgumentType::$variant))|*
                )
            }
        }

        pub trait FromParsedValue: Sized {
            fn from_parsed(val: &ParsedValue, name: &str) -> Self;
        }

        $(
            impl FromParsedValue for $type {
                fn from_parsed(val: &ParsedValue, name: &str) -> Self {
                    match val {
                        ParsedValue::$variant(v) => v.clone(),
                        _ => unreachable!(
                            "Argument '{name}' is not of the expected type"
                        )
                    }
                }
            }
        )*
    };
}

define_arg_types! {
    Integer => i64,
    Text    => String,
    Boolean => bool,
}

#[macro_export]
macro_rules! strong_type {
    (
        $name:ident, $inner:ty
        $(, $meta:meta)*
    ) => {
        $(#[derive($meta)])*
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub struct $name($inner);

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self(v)
            }
        }

        impl From<$name> for $inner {
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl std::convert::AsRef<$inner> for $name {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }

        impl std::convert::AsMut<$inner> for $name {
            fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let value = s.parse::<$inner>().map_err(|e| e.to_string())?;
                Ok(value.into())
            }
        }
    };
}

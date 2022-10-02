#[derive(Debug, Clone, Copy)]
pub enum YesNo {
    No,
    Yes,
}

impl Default for YesNo {
    fn default() -> Self {
        Self::No
    }
}

impl YesNo {
    /// Returns `true` if the yes no is [`Yes`].
    ///
    /// [`Yes`]: YesNo::Yes
    #[must_use]
    pub fn is_yes(&self) -> bool {
        matches!(self, Self::Yes)
    }

    /// Returns `true` if the yes no is [`No`].
    ///
    /// [`No`]: YesNo::No
    #[must_use]
    pub fn is_no(&self) -> bool {
        matches!(self, Self::No)
    }
}

impl sqlx::Type<sqlx::postgres::Postgres> for YesNo {
    fn type_info() -> <sqlx::postgres::Postgres as sqlx::Database>::TypeInfo {
        <&str as sqlx::Type<sqlx::postgres::Postgres>>::type_info()
    }

    fn compatible(ty: &<sqlx::postgres::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <&str as sqlx::Type<sqlx::postgres::Postgres>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::postgres::Postgres> for YesNo {
    fn decode(
        value: <sqlx::postgres::Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let res = value.as_str()?;
        let res = match res {
            "YES" => YesNo::Yes,
            "NO" => YesNo::No,
            _ => Err(anyhow!("invalid yes/no value {}", res))?,
        };

        Ok(res)
    }
}

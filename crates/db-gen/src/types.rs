// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ChannelType {
    telegram,
}
impl<'a> postgres_types::ToSql for ChannelType {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            ChannelType::telegram => "telegram",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "channel_type" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 1 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "telegram" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for ChannelType {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<ChannelType, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "telegram" => Ok(ChannelType::telegram),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "channel_type" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 1 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "telegram" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ChannelMessageDirection {
    inbound,
    outbound,
}
impl<'a> postgres_types::ToSql for ChannelMessageDirection {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            ChannelMessageDirection::inbound => "inbound",
            ChannelMessageDirection::outbound => "outbound",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "channel_message_direction" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 2 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "inbound" => true,
                    "outbound" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for ChannelMessageDirection {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<ChannelMessageDirection, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "inbound" => Ok(ChannelMessageDirection::inbound),
            "outbound" => Ok(ChannelMessageDirection::outbound),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "channel_message_direction" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 2 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "inbound" => true,
                    "outbound" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ChannelMessageStatus {
    pending,
    processing,
    processed,
    sent,
    failed,
}
impl<'a> postgres_types::ToSql for ChannelMessageStatus {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            ChannelMessageStatus::pending => "pending",
            ChannelMessageStatus::processing => "processing",
            ChannelMessageStatus::processed => "processed",
            ChannelMessageStatus::sent => "sent",
            ChannelMessageStatus::failed => "failed",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "channel_message_status" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "pending" => true,
                    "processing" => true,
                    "processed" => true,
                    "sent" => true,
                    "failed" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for ChannelMessageStatus {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<ChannelMessageStatus, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "pending" => Ok(ChannelMessageStatus::pending),
            "processing" => Ok(ChannelMessageStatus::processing),
            "processed" => Ok(ChannelMessageStatus::processed),
            "sent" => Ok(ChannelMessageStatus::sent),
            "failed" => Ok(ChannelMessageStatus::failed),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "channel_message_status" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "pending" => true,
                    "processing" => true,
                    "processed" => true,
                    "sent" => true,
                    "failed" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ResourceVisibility {
    private,
    org,
}
impl<'a> postgres_types::ToSql for ResourceVisibility {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            ResourceVisibility::private => "private",
            ResourceVisibility::org => "org",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "resource_visibility" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 2 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "private" => true,
                    "org" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for ResourceVisibility {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<ResourceVisibility, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "private" => Ok(ResourceVisibility::private),
            "org" => Ok(ResourceVisibility::org),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "resource_visibility" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 2 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "private" => true,
                    "org" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}

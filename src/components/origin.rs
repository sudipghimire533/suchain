use serde::Serialize;
use crate::components::AccountId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Origin {
    Signed(AccountId),
    Root,
    Unsigned,
}

impl Origin {
    pub fn signed(&self) -> Option<&AccountId> {
        if let Origin::Signed(account_id) = self {
            Some(account_id)
        } else {
            None
        }
    }

    pub fn root(&self) -> Option<()> {
        if self == &Origin::Root {
            Some(())
        } else {
            None
        }
    }

    pub fn unsigned(&self) -> Option<()> {
        if self == &Origin::Unsigned {
            Some(())
        } else {
            None
        }
    }
}

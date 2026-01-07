pub mod query {
    use rebrickable_database_api::*;

    use derive_more::From;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, From)]
    pub enum GetItem {
        PartFromId(PartId),
        PartFromName(PartName),
        ColorFromId(ColorId),
        ColorFromName(ColorName),
        Element(ElementId),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum FindItem {
        PartId,
        PartName,
        ColorId,
        ColorName,
        Element,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Query {
        Get(GetItem),
        Find(FindItem),
    }

    impl<T: Into<GetItem>> From<T> for Query {
        fn from(value: T) -> Self {
            Query::Get(value.into())
        }
    }

    impl From<FindItem> for Query {
        fn from(value: FindItem) -> Self {
            Query::Find(value)
        }
    }
}

pub mod response {
    use rebrickable_database_api::*;

    use derive_more::From;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, From)]
    pub enum GetItemResponse {
        Part(Part),
        Color(Color),
        Element(Element),
        NotFound,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, From)]
    pub enum IterItemsResponse {
        PartId(PartId),
        PartName(PartName),
        ColorId(ColorId),
        ColorName(ColorName),
        ElementId(ElementId),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Response {
        GetItem(GetItemResponse, crate::query::GetItem),
        /// The Option is None once the stream has ended.
        IterItems(Option<IterItemsResponse>),
    }

    impl<T: Into<IterItemsResponse>> From<T> for Response {
        fn from(value: T) -> Self {
            Response::IterItems(Some(value.into()))
        }
    }
}

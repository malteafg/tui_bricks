pub mod query {
    use rebrickable_database_api::*;

    use bincode::{Decode, Encode};
    use derive_more::From;

    #[derive(Debug, Clone, Decode, Encode, From)]
    pub enum GetItem {
        PartFromId(PartId),
        PartFromName(PartName),
        ColorFromId(ColorId),
        ColorFromName(ColorName),
        Element(ElementId),
    }

    #[derive(Debug, Clone, Decode, Encode)]
    pub enum FindItem {
        PartId,
        PartName,
        ColorId,
        ColorName,
        Element,
    }

    #[derive(Debug, Clone, Decode, Encode)]
    pub enum Query {
        Get(GetItem),
        Find(FindItem),
    }

    impl<T: Into<GetItem>> From<T> for Query {
        fn from(value: T) -> Self {
            Query::Get(value.into())
        }
    }
}

pub mod response {
    use rebrickable_database_api::*;

    use bincode::{Decode, Encode};
    use derive_more::From;

    #[derive(Debug, Clone, Encode, Decode, From)]
    pub enum GetItemResponse {
        Part(Part),
        Color(Color),
        Element(Element),
        NotFound,
    }

    #[derive(Debug, Clone, Encode, Decode, From)]
    pub enum IterItemsResponse {
        PartId(PartId),
        PartName(PartName),
        ColorId(ColorId),
        ColorName(ColorName),
        ElementId(ElementId),
    }

    #[derive(Debug, Clone, Encode, Decode, From)]
    pub enum Response {
        GetItem(GetItemResponse, crate::query::GetItem),
        /// The Option is None once the stream has ended.
        IterItems(Option<IterItemsResponse>),
    }
}
